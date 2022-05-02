defmodule Lsm.C0 do
  @moduledoc """
  C0 in the LSM is the in-memory dictionary, that is used for fast access to the stored data.
  This is where the LSM will look first to insert and retrieve data.
  """
  use GenServer
  require Logger

  def start_link(init_arg) do
    Logger.info("LSM::C0 starting")
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
