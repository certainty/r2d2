defmodule Lsm.Supervisor do
  @moduledoc """
  Supervisor for the logstructured merge tree.
  This is used to store the data locally on a node
  """
  use Supervisor
  require Logger

  @spec start_link(any) :: :ignore | {:error, any} | {:ok, pid}
  def start_link(init_arg) do
    Logger.info("#{__MODULE__} starting")
    Supervisor.start_link(__MODULE__, init_arg, name: __MODULE__)
  end

  def init(storage_options) do
    children = [
      {Lsm.C0, []},
      {Lsm.C1.Supervisor, storage_options}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end
end
