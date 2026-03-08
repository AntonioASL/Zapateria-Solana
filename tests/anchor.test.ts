import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Zapateria } from "../target/types/zapateria";
import { assert } from "chai";

describe("zapateria", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Zapateria as Program<Zapateria>;

  const owner = provider.wallet;

  let zapateriaPDA: anchor.web3.PublicKey;

  before(async () => {

    [zapateriaPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("zapateria"),
        owner.publicKey.toBuffer()
      ],
      program.programId
    );

  });

  it("Crea una zapateria", async () => {

    await program.methods
      .crearZapateria("Zapateria Test")
      .accounts({
        owner: owner.publicKey,
        zapateria: zapateriaPDA,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .rpc();

    const cuenta = await program.account.zapateria.fetch(zapateriaPDA);

    assert.equal(cuenta.nombre, "Zapateria Test");

  });

  it("Agrega un zapato", async () => {

    await program.methods
      .agregarZapato("Adidas", 26, 1200)
      .accounts({
        owner: owner.publicKey,
        zapateria: zapateriaPDA
      })
      .rpc();

    const cuenta = await program.account.zapateria.fetch(zapateriaPDA);

    assert.equal(cuenta.zapatos.length, 1);

  });

  it("Cambia disponibilidad", async () => {

    await program.methods
      .alternarEstado("Adidas")
      .accounts({
        owner: owner.publicKey,
        zapateria: zapateriaPDA
      })
      .rpc();

    const cuenta = await program.account.zapateria.fetch(zapateriaPDA);

    assert.equal(cuenta.zapatos[0].disponible, false);

  });

});
