package service

import (
	"github.com/KCraft-v1/auth-service/src/repo"
)

type Option func(*AuthService)

func WithRepo(repo repo.AuthRepo) Option {
	return func(service *AuthService) {
		service.repo = repo
	}
}
