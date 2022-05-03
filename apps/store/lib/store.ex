require Logger

defmodule Store do
  @moduledoc """
  Documentation for `Store`.
  """
  use Application

  def start(_type, _args) do
    Logger.info("#{__MODULE__} starting")

    {:ok, lsm_opts} = lsm_options()

    children = [
      {Node.Supervisor, %{:lsm => lsm_opts}}
    ]

    Supervisor.start_link(children, strategy: :one_for_one)
  end

  defp lsm_options do
    {:ok, Application.fetch_env!(:store, :storage)}
  end
end
