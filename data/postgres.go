package data

import (
	"context"
	"errors"
	"fmt"
	"mime/multipart"
	"tranquility/models"
	"tranquility/services"

	"github.com/jmoiron/sqlx"
	_ "github.com/lib/pq"
)

var (
	ErrAttachmentNotFound = errors.New("attachment was not found while deleting")
)

type Postgres struct {
	authRepo
	attachmentRepo
	guildRepo
	fileHandler *services.FileHandler
}

func CreatePostgres(connectionString string, fileHandler *services.FileHandler) (*Postgres, error) {
	db, err := sqlx.Connect("postgres", "user=postgres password=server dbname=tranquility sslmode=disable")
	if err != nil {
		return nil, err
	}

	return &Postgres{
		authRepo:       authRepo{db},
		attachmentRepo: attachmentRepo{db},
		guildRepo:      guildRepo{db},
		fileHandler:    fileHandler,
	}, nil
}

func (p *Postgres) Login(ctx context.Context, user *models.AuthUser) (*models.AuthUser, error) {
	if user.Password == "" {
		return nil, ErrMissingPassword
	}

	credentials, err := p.authRepo.Login(ctx, user)
	if err != nil {
		return nil, err
	}

	if ok, err := services.VerifyPassword(user.Password, credentials.Password); err != nil {
		return nil, fmt.Errorf("an error occurred while verifying password: %v", err)
	} else if !ok {
		return nil, ErrInvalidCredentials
	}

	authToken, err := services.GenerateToken(credentials)
	if err != nil {
		return nil, fmt.Errorf("an error occurred while generating token: %v", err)
	}
	credentials.Token = authToken
	credentials.ClearAuth()

	return credentials, nil
}

func (p *Postgres) Register(ctx context.Context, user *models.AuthUser) (*models.AuthUser, error) {
	if user.Password == "" || user.ConfirmPassword == "" {
		return nil, ErrInvalidCredentials
	}

	password, err := services.HashPassword(user.Password)
	if err != nil {
		return nil, fmt.Errorf("an error occurred hashing password while registering user: %v", err)
	}

	user.Password = password
	output, err := p.authRepo.Register(ctx, user)
	if err != nil {
		return nil, fmt.Errorf("an error occurred while registering user: %v", err)
	}

	return output, nil
}

func (p *Postgres) RefreshToken(ctx context.Context, user *models.AuthUser) (*models.AuthUser, error) {
	if user.ID == 0 || user.RefreshToken == "" {
		return nil, ErrInvalidCredentials
	}

	credentials, err := p.authRepo.RefreshToken(ctx, user)
	if err != nil {
		return nil, err
	}

	token, err := services.GenerateToken(credentials)
	if err != nil {
		return nil, err
	}
	credentials.Token = token
	return credentials, nil
}

func (p *Postgres) CreateAttachment(ctx context.Context, file *multipart.File, attachment *models.Attachment) (*models.Attachment, error) {
	outputName, outputPath, err := p.fileHandler.StoreFile(file, attachment.FileName)
	if err != nil {
		return nil, err
	}

	attachment.FileName = outputName
	attachment.FilePath = outputPath

	output, err := p.attachmentRepo.CreateAttachment(ctx, attachment)
	if err != nil {
		return nil, err
	}

	return output, nil
}

func (p *Postgres) DeleteAttachment(ctx context.Context, fileId, userId int32) error {
	transaction, fileName, err := p.attachmentRepo.DeleteAttachment(ctx, fileId, userId)
	if err != nil {
		return err
	}
	if fileName == "" {
		return ErrAttachmentNotFound
	}

	err = p.fileHandler.DeleteFile(fileName)
	if err != nil {
		return err
	}

	transaction.Commit()
	return nil
}

func (p *Postgres) GetJoinedGuilds(ctx context.Context, userId int32) ([]models.Guild, error) {
	guilds, err := p.guildRepo.GetJoinedGuilds(ctx, userId)
	if err != nil {
		return nil, err
	}

	for i := range guilds {
		channels, err := p.guildRepo.GetGuildChannels(ctx, guilds[i].ID, userId)
		if err != nil {
			return nil, err
		}
		guilds[i].Channels = channels
	}

	return guilds, nil
}

func (p *Postgres) CreateGuild(ctx context.Context, guild *models.Guild, userId int32) (*models.Guild, error) {
	tx, guild, err := p.guildRepo.CreateGuild(ctx, guild, userId)
	if err != nil {
		return nil, err
	}

	if err = p.guildRepo.AddGuildMember(ctx, guild.ID, userId, tx); err != nil {
		if rbErr := tx.Rollback(); rbErr != nil {
			return nil, fmt.Errorf("rollback error: %v, original error: %v", rbErr, err)
		}
		return nil, err
	}

	if err = tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %v", err)
	}

	return guild, nil
}
