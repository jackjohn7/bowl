package routes

import "github.com/labstack/echo/v4"

func IndexRoute(e *echo.Echo) {
	e.GET("/index", func(c echo.Context) error {
		return c.String(200, "YOU HIT THE INDEX ROUTE")
	})
}
