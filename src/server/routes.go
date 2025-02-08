package server

import "net/http"

func (s *Server) registerRoutes() {
	s.router.Handle("/register", s.HandleRegister()).Methods(http.MethodPost)
}
