defmodule Node.Supervisor do
  @moduledoc """
  The root of the node's supervision hierarchy.
  Here we plug in the storage subsystem as well as the functionality to communicate within the cluster.
  """
  use Supervisor
  require Logger

  @spec start_link(any) :: :ignore | {:error, any} | {:ok, pid}
  def start_link(init_arg) do
    Logger.info("Node::Supervisor starting")
    Supervisor.start_link(__MODULE__, init_arg, name: __MODULE__)
  end

  @impl true
  def init(%{lsm: lsm_opts} = arg) do
    children = [
      {Lsm.Supervisor, [lsm_opts]}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end
end
