defmodule PasswdExTest do
  use ExUnit.Case
  doctest PasswdEx

  test "getuid returns a uid" do
    value = PasswdEx.getuid()

    case :os.type() do
      {:unix, _} -> assert match?({:ok, uid} when uid >= 0, value)
      _ -> assert match?({:error, :unsupported}, value)
    end
  end

  test "getgid returns a gid" do
    value = PasswdEx.getgid()

    case :os.type() do
      {:unix, _} -> assert match?({:ok, uid} when uid >= 0, value)
      _ -> assert match?({:error, :unsupported}, value)
    end
  end

  test "geteuid returns a uid" do
    value = PasswdEx.geteuid()

    case :os.type() do
      {:unix, _} -> assert match?({:ok, uid} when uid >= 0, value)
      _ -> assert match?({:error, :unsupported}, value)
    end
  end

  test "getegid returns a gid" do
    value = PasswdEx.getegid()

    case :os.type() do
      {:unix, _} -> assert match?({:ok, uid} when uid >= 0, value)
      _ -> assert match?({:error, :unsupported}, value)
    end
  end

  test "getpwuid returns a struct" do
    value = PasswdEx.getpwuid(0)

    case :os.type() do
      {:unix, _} -> assert match?({:ok, %PasswdEx.Passwd{uid: 0}}, value)
      _ -> assert match?({:error, :unsupported}, value)
    end
  end

  test "getpwnam returns a struct" do
    value = PasswdEx.getpwnam("root")

    case :os.type() do
      {:unix, _} -> assert match?({:ok, %PasswdEx.Passwd{uid: 0}}, value)
      _ -> assert match?({:error, :unsupported}, value)
    end
  end
end
