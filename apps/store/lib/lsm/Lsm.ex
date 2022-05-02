defmodule Lsm.Lsm do
  @moduledoc """
  The main interface to the LSM (https://en.wikipedia.org/wiki/Log-structured_merge-tree)

  Supervision hierarchy and architecture:
  -----------------------------------------

  The system is composed of two main components C0, which is the in memory representation and C1 which
  is the on disk representation. The on disk representation can be broken down further (and potentially bundled in its own application),
  as it contains the storage for in the form of sorted string tables as well as compaction on those tables.
  """
end
