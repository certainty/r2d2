require Logger

defmodule Lsm.Wal do
  @moduledoc """
  Write ahead log for the LSM.
  """
  use GenServer

  def start_link(init_arg) do
    Logger.info("LSM::WAL starting")
    GenServer.start_link(__MODULE__, init_arg, name: __MODULE__)
  end

  @impl true
  def init(_) do
    {:ok, nil}
  end

  @impl true
  def handle_call(_, _, _) do
    {:reply, nil, nil}
  end
end
