require Logger

defmodule Lsm.C1 do
  @moduledoc """
  This is the C1 system of the LSM. It is durable on disk storage in the form of sorted string tables, that
  egt compacted over time
  """
  use GenServer

  def start_link(init_arg) do
    Logger.info("LSM::C1 starting")
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
