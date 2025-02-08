package server

import (
	"context"
	"net/http"
	"time"

	"github.com/KCraft-v1/auth-service/src/service"
	"github.com/gorilla/mux"
)

type Server struct {
	router           *mux.Router
	http             *http.Server
	httpreadtimeout  time.Duration
	httpwritetimeout time.Duration

	authService *service.AuthService
}

func NewServer(ctx context.Context, options ...Option) *Server {
	s := &Server{
		router: mux.NewRouter(),
	}

	for _, option := range options {
		option(s)
	}

	if s.authService == nil {
		panic("auth service is required")
	}

	s.registerRoutes()
	return s

}

func (s *Server) ListenAndServe(listenAddr string) error {
	s.http = &http.Server{
		Addr:         listenAddr,
		Handler:      s.router,
		ReadTimeout:  s.httpreadtimeout,
		WriteTimeout: s.httpwritetimeout,
	}

	return s.http.ListenAndServe()
}
