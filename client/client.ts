import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Zapateria } from "../target/types/zapateria";

async function main() {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Zapateria as Program<Zapateria>;

  const owner = provider.wallet;

  // PDA de la zapateria
  const [zapateriaPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("zapateria"),
      owner.publicKey.toBuffer()
    ],
    program.programId
  );

  console.log("Zapateria PDA:", zapateriaPDA.toString());

  // Crear Zapateria
  await program.methods
    .crearZapateria("Zapateria Blockchain")
    .accounts({
      owner: owner.publicKey,
      zapateria: zapateriaPDA,
      systemProgram: anchor.web3.SystemProgram.programId
    })
    .rpc();

  console.log("Zapateria creada");

  // Agregar zapato
  await program.methods
    .agregarZapato("Nike Air", 27, 1500)
    .accounts({
      owner: owner.publicKey,
      zapateria: zapateriaPDA
    })
    .rpc();

  console.log("Zapato agregado");

  // Ver zapatos
  await program.methods
    .verZapatos()
    .accounts({
      owner: owner.publicKey,
      zapateria: zapateriaPDA
    })
    .rpc();

  console.log("Lista mostrada en logs");

}

main();
