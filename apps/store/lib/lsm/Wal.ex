require Logger

defmodule Lsm.Wal do
  @moduledoc """
  Write ahead log for the LSM.
  """
  use GenServer

  def start_link(init_arg) do
    Logger.info("#{__MODULE__} starting")
    GenServer.start_link(__MODULE__, init_arg, name: __MODULE__)
  end

  @spec write(:set | :get, binary, binary) :: any
  def write(:set, key, value) do
    GenServer.call(__MODULE__, {:write, {:set, key, value}})
  end

  def write(:del, key) do
    GenServer.call(__MODULE__, {:write, {:del, key}})
  end

  @impl true
  def init([%{base_path: base_path}]) do
    # make sure the base path exists
    with :ok <- File.mkdir_p(base_path),
         {:ok, file} = Lsm.Wal.File.open(Path.join(base_path, "wal.bin")) do
      {:ok, %{file: file}}
    end
  end

  @impl true
  def handle_call({:write, op}, _from, %{file: file} = state) do
    with {:ok, encoded} <- encode_op(op, file.version),
         :ok <- Lsm.Wal.File.write(file, encoded) do
      {:reply, :ok, state}
    end
  end

  defp encode_op(op, 0) do
    Bertex.encode(op)
  end

  defp encode_op(_op, version) do
    {:error, {"Unsupported version", version}}
  end
end
