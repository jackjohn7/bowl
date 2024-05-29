package main

import (
	"net/http"

	"github.com/labstack/echo/v4"

	"go_echo_example/routes"
)

func main() {
	e := echo.New()
	e.GET("/", func(c echo.Context) error {
		return c.String(http.StatusOK, "Hello, World!")
	})
	routes.IndexRoute(e)
	e.Logger.Fatal(e.Start(":1323"))
}
