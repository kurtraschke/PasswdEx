case PasswdEx.getuid() do
  {:ok, uid} ->
    IO.inspect(uid, label: "uid")
    case PasswdEx.getpwuid(uid) do
      {:ok, %PasswdEx.Passwd{} = passwd} -> IO.inspect(passwd, label: "passwd")
      {:error, _} = error -> IO.inspect(error, label: "getpwuid error")
    end
  {:error, _} = error -> IO.inspect(error, label: "getuid error")
end
