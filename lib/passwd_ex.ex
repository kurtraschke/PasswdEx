defmodule PasswdEx do
  @moduledoc """
  Documentation for `PasswdEx`.
  """

  @doc """
  Hello world.

  """

  use Rustler, otp_app: :passwd_ex, crate: "passwdex"

  defmodule Passwd do
    @type t :: %__MODULE__{
            name: String.t(),
            passwd: String.t(),
            uid: non_neg_integer(),
            gid: non_neg_integer(),
            gecos: String.t(),
            dir: String.t(),
            shell: String.t()
          }
    defstruct [:name, :passwd, :uid, :gid, :gecos, :dir, :shell]
  end

  @spec getuid :: non_neg_integer()
  def getuid(), do: :erlang.nif_error(:nif_not_loaded)

  @spec getgid :: non_neg_integer()
  def getgid(), do: :erlang.nif_error(:nif_not_loaded)

  @spec geteuid :: non_neg_integer()
  def geteuid(), do: :erlang.nif_error(:nif_not_loaded)

  @spec getegid :: non_neg_integer()
  def getegid(), do: :erlang.nif_error(:nif_not_loaded)

  @spec getpwuid(non_neg_integer()) :: Passwd.t() | nil
  def getpwuid(_uid), do: :erlang.nif_error(:nif_not_loaded)

  @spec getpwnam(String.t()) :: Passwd.t() | nil
  def getpwnam(_name), do: :erlang.nif_error(:nif_not_loaded)
end
