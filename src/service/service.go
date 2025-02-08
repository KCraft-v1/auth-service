package service

import (
	"github.com/KCraft-v1/auth-service/src/repo"
)

type AuthService struct {
	repo repo.AuthRepo
}

func NewAuthService(options ...Option) *AuthService {

	service := &AuthService{}

	for _, option := range options {
		option(service)
	}

	return service
}
