defmodule PasswdExTest do
  use ExUnit.Case
  doctest PasswdEx

  test "getuid returns a uid" do
    assert PasswdEx.getuid() >= 0
  end

  test "getgid returns a gid" do
    assert PasswdEx.getgid() >= 0
  end

  test "geteuid returns a uid" do
    assert PasswdEx.geteuid() >= 0
  end

  test "getegid returns a gid" do
    assert PasswdEx.getegid() >= 0
  end

  test "getpwuid returns a struct" do
    assert match?(%PasswdEx.Passwd{uid: 0}, PasswdEx.getpwuid(0))
  end

  test "getpwnam returns a struct" do
    assert match?(%PasswdEx.Passwd{uid: 0}, PasswdEx.getpwnam("root"))
  end
end
