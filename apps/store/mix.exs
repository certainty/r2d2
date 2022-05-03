defmodule Store.MixProject do
  use Mix.Project

  def project do
    [
      app: :store,
      version: "0.1.0",
      build_path: "../../_build",
      config_path: "../../config/config.exs",
      deps_path: "../../deps",
      lockfile: "../../mix.lock",
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger, :eternal],
      env: [node: %{:listen => "127.0.0.1:4000"}, storage: %{}],
      mod: {Store, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:bertex, "~> 1.3"},
      {:eternal, "~> 1.2"}
    ]
  end
end
