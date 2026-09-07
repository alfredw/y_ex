defmodule Yex.NervPackagingTest do
  use ExUnit.Case, async: true

  test "the packaged native library exposes document inspection without a fixture loader" do
    doc = Yex.Doc.with_options(%Yex.Doc.Options{offset_kind: :utf16})
    text = Yex.Doc.get_text(doc, "markdown")
    assert :ok = Yex.Text.insert(text, 0, "Packaged 😀")
    assert Yex.Text.to_string(text) == "Packaged 😀"

    assert {:ok, json} = Yex.Nif.inspect_document(doc)
    assert %{"version" => 1, "roots" => ["markdown"]} = Jason.decode!(json)

    state = Yex.encode_state_as_update!(doc)
    assert {:ok, json} = Yex.Nif.inspect_update(state, false)
    assert %{"version" => 1, "blocks" => [_ | _]} = Jason.decode!(json)
  end

  @tag :tmp_dir
  test "the packaged application loads after relocation without the source checkout", %{
    tmp_dir: dir
  } do
    app = Path.join(dir, "y_ex")
    File.mkdir_p!(app)
    File.cp_r!(Application.app_dir(:y_ex, "ebin"), Path.join(app, "ebin"))

    File.cp_r!(Application.app_dir(:y_ex, "priv"), Path.join(app, "priv"),
      dereference_symlinks: true
    )

    expression = "doc = Yex.Doc.new(); {:ok, _} = Yex.Nif.inspect_document(doc)"
    args = ["--erl", "+S 2:2", "-pa", Path.join(app, "ebin"), "-e", expression]
    opts = [cd: dir, env: [{"NERV_SCHEMA_NATIVE", nil}], stderr_to_stdout: true]
    assert {_output, 0} = System.cmd(System.find_executable("elixir"), args, opts)

    File.rm_rf!(Path.join(app, "priv/native"))
    assert {_output, status} = System.cmd(System.find_executable("elixir"), args, opts)
    assert status != 0
  end
end
