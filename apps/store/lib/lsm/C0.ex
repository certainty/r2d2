defmodule Lsm.C0 do
  @moduledoc """
  C0 in the LSM is the in-memory dictionary, that is used for fast access to the stored data.
  This is where the LSM will look first to insert and retrieve data.
  """

  require Logger

  @tombstone :t

  def start_link(_) do
    Logger.info("#{__MODULE__} starting")
    # create the ETS table that is used for the in-memory dictionary
    Eternal.start_link(:memtable, [:ordered_set, write_concurrency: true])
  end

  @doc """
  Inserts or updates a key-value pair in the in-memory dictionary.
  """
  @spec insert(binary, binary) :: :ok | {:error, any}
  def insert(key, value) when is_binary(key) and is_binary(value) do
    update(key, value)
  end

  def insert(_, _) do
    {:error, "key and value must be binary"}
  end

  @doc """
  Deletes the value for the given key if it existed before
  """
  @spec delete(binary) :: :ok | :not_found
  def delete(key) when is_binary(key) do
    case lookup(key) do
      :deleted -> :ok
      {:found, _} -> update(key, @tombstone)
      _ -> :not_found
    end
  end

  def delete(_) do
    {:error, "key must be binary"}
  end

  @doc """
  Looks up the value for the given key.
  If the key does not exist, it returns :not_found.
  If the key exists and has been deleted, it returns :deleted.
  Otherwise, it returns the value.
  """
  @spec lookup(any) :: :deleted | :not_found | {:found, any} | {:error, any}
  def lookup(key) when is_binary(key) do
    case :ets.lookup(:memtable, key) do
      [{_, @tombstone}] -> :deleted
      [{_, value}] -> {:found, value}
      _ -> :not_found
    end
  end

  def lookup(_) do
    {:error, "key must be binary"}
  end

  def child_spec(opts) do
    %{
      id: __MODULE__,
      start: {__MODULE__, :start_link, [opts]}
    }
  end

  defp update(key, value) do
    :ets.insert(:memtable, {key, value})
    :ok
  end
end
