package main

import (
	"context"
	"os/signal"
	"syscall"

	"github.com/KCraft-v1/auth-service/src/repo"
	"github.com/KCraft-v1/auth-service/src/server"
	"github.com/KCraft-v1/auth-service/src/service"
)

func main() {
	ctx, cancel := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer cancel()

	authService := service.NewAuthService(service.WithRepo(repo.NewPostgresRepo(nil)))

	s := server.NewServer(ctx,
		server.WithAuthService(authService),
	)

	s.ListenAndServe(":8080")
}
