defmodule Lsm.Lsm do
  @moduledoc """
  The main interface to the LSM (https://en.wikipedia.org/wiki/Log-structured_merge-tree)

  Supervision hierarchy and architecture:
  -----------------------------------------

  The system is composed of two main components C0, which is the in memory representation and C1 which
  is the on disk representation. The on disk representation can be broken down further (and potentially bundled in its own application),
  as it contains the storage for in the form of sorted string tables as well as compaction on those tables.
  """

  def insert(_key, _value) do
    :ok
  end

  def delete(_key) do
    :ok
  end

  @spec lookup(binary) :: :not_found | {:found, any}
  def lookup(key) do
    case Lsm.C0.lookup(key) do
      :deleted -> :not_found
      # TODO: check in C1 instead
      :not_found -> :not_found
      {:found, value} -> {:found, value}
    end
  end
end
