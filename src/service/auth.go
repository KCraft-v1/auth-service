package service

import (
	"github.com/KCraft-v1/auth-service/src/repo"
)

type AuthService struct {
	repo repo.AuthRepo
}

func NewAuthService(ur repo.AuthRepo) *AuthService {
	return &AuthService{
		repo: ur,
	}
}
