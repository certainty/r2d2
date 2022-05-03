defmodule Lsm.Wal.File do
  require Logger

  defmodule Header do
    defstruct version: 0

    @stanza "R2D2.WAL"
    @stanza_bytes 8
    @version_bytes 4
    @version_bits @version_bytes * 8
    @padding_bytes 12
    @padding_bits @padding_bytes * 8
    @header_size @stanza_bytes + @version_bytes + @padding_bytes

    def current do
      %__MODULE__{version: 0}
    end

    def read(file) do
      case IO.binread(file, @header_size) do
        :eof -> {:error, "EOF while reading header"}
        {:error, details} -> {:error, {"invalid header", details}}
        data -> decode(data)
      end
    end

    def write(file, header) do
      IO.binwrite(file, [@stanza, header.version, <<0::size(@padding_bits)>>])
    end

    defp decode(binary) do
      case binary do
        <<@stanza, v::size(@version_bits), _reserved::binary>> -> {:ok, %__MODULE__{version: v}}
        other -> {:error, {"invalid header", "invalid header stanza", other}}
      end
    end
  end

  defstruct header: nil, io: nil

  def open(file_path) do
    if File.exists?(file_path) do
      open_file(file_path)
    else
      create_file(file_path)
    end
  end

  def write(file, encoded_op) do
    IO.binwrite(file.io, encoded_op)
  end

  @mode [:append]
  defp create_file(file_path) do
    header = Header.current()

    with {:ok, file} <- File.open(file_path, @mode),
         :ok <- Header.write(file, header) do
      Logger.info("WAL created at #{file_path} with version: #{header.version}")
      {:ok, %__MODULE__{header: header, io: file}}
    end
  end

  defp open_file(file_path) do
    with {:ok, header} <- read_header(file_path),
         {:ok, file} <- File.open(file_path, @mode) do
      Logger.info("WAL opened at #{file_path} with version: #{header.version}")
      {:ok, %__MODULE__{header: header, io: file}}
    end
  end

  defp read_header(file_path) do
    with {:ok, file} <- File.open(file_path, [:read]),
         {:ok, header} <- Header.read(file),
         :ok <- File.close(file) do
      {:ok, header}
    end
  end
end
