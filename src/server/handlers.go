package server

import "net/http"

func (s *Server) HandleRegister() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Write([]byte("Register"))
	}
}
