defmodule Lsm.C1.Supervisor do
  use Supervisor
  require Logger

  def start_link(init_arg) do
    Logger.info("#{__MODULE__} starting")
    Supervisor.start_link(__MODULE__, init_arg, name: __MODULE__)
  end

  def init(storage_options) do
    children = []

    Supervisor.init(children, strategy: :one_for_all)
  end
end
