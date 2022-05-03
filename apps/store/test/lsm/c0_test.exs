defmodule Lsm.C0Test do
  use ExUnit.Case, async: true

  setup _context do
    _ = start_supervised(Lsm.C0)
    :ok
  end

  test "insert data" do
    assert Lsm.C0.insert("key1", "value1") == :ok
  end

  test "lookup data that exists" do
    Lsm.C0.insert("key1", "value1")
    assert Lsm.C0.lookup("key1") == {:found, "value1"}
  end

  test "lookup data that does not exist" do
    assert Lsm.C0.lookup("nope") == :not_found
  end

  test "lookup deleted item" do
    Lsm.C0.insert("key1", "value1")
    Lsm.C0.delete("key1")

    assert Lsm.C0.lookup("key1") == :deleted
  end
end
