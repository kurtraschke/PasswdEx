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

  @spec getuid :: {:ok, non_neg_integer()} | {:error, atom()}
  def getuid(), do: :erlang.nif_error(:nif_not_loaded)

  @spec getgid :: {:ok, non_neg_integer()} | {:error, atom()}
  def getgid(), do: :erlang.nif_error(:nif_not_loaded)

  @spec geteuid :: {:ok, non_neg_integer()} | {:error, atom()}
  def geteuid(), do: :erlang.nif_error(:nif_not_loaded)

  @spec getegid :: {:ok, non_neg_integer()} | {:error, atom()}
  def getegid(), do: :erlang.nif_error(:nif_not_loaded)

  @spec getpwuid(non_neg_integer()) :: {:ok, Passwd.t()} | {:error, atom()}
  def getpwuid(_uid), do: :erlang.nif_error(:nif_not_loaded)

  @spec getpwnam(String.t()) :: {:ok, Passwd.t()} | {:error, atom()}
  def getpwnam(_name), do: :erlang.nif_error(:nif_not_loaded)
end
