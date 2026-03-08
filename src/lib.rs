use anchor_lang::prelude::*;

// ID del Solana Program, este espacio se llena automaticamente al hacer el "build"
declare_id!("");

#[program]
pub mod zapateria {
    use super::*;

    //////////////////////////// Instruccion: Crear Zapateria /////////////////////////////////////

    pub fn crear_zapateria(context: Context<NuevaZapateria>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let zapatos: Vec<Zapato> = Vec::new();

        context.accounts.zapateria.set_inner(Zapateria {
            owner: owner_id,
            nombre,
            zapatos,
        });

        Ok(())
    }

    //////////////////////////// Instruccion: Agregar Zapato /////////////////////////////////////

    pub fn agregar_zapato(
        context: Context<NuevoZapato>,
        nombre: String,
        talla: u8,
        precio: u16,
    ) -> Result<()> {

        require!(
            context.accounts.zapateria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let zapato = Zapato {
            nombre,
            talla,
            precio,
            disponible: true,
        };

        context.accounts.zapateria.zapatos.push(zapato);

        Ok(())
    }

    //////////////////////////// Instruccion: Eliminar Zapato /////////////////////////////////////

    pub fn eliminar_zapato(context: Context<NuevoZapato>, nombre: String) -> Result<()> {

        require!(
            context.accounts.zapateria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let zapatos = &mut context.accounts.zapateria.zapatos;

        for i in 0..zapatos.len() {
            if zapatos[i].nombre == nombre {
                zapatos.remove(i);
                msg!("Zapato {} eliminado!", nombre);
                return Ok(());
            }
        }

        Err(Errores::ZapatoNoExiste.into())
    }

    //////////////////////////// Instruccion: Ver Zapatos /////////////////////////////////////

    pub fn ver_zapatos(context: Context<NuevoZapato>) -> Result<()> {

        require!(
            context.accounts.zapateria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "La lista de zapatos actualmente es: {:#?}",
            context.accounts.zapateria.zapatos
        );

        Ok(())
    }

    //////////////////////////// Instruccion: Alternar Estado /////////////////////////////////////

    pub fn alternar_estado(context: Context<NuevoZapato>, nombre: String) -> Result<()> {

        require!(
            context.accounts.zapateria.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let zapatos = &mut context.accounts.zapateria.zapatos;

        for i in 0..zapatos.len() {

            let estado = zapatos[i].disponible;

            if zapatos[i].nombre == nombre {

                let nuevo_estado = !estado;
                zapatos[i].disponible = nuevo_estado;

                msg!(
                    "El zapato: {} ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        Err(Errores::ZapatoNoExiste.into())
    }
}

//////////////////////////// Codigos de error /////////////////////////////////////

#[error_code]
pub enum Errores {

    #[msg("Error, no eres el propietario de la zapateria que deseas modificar")]
    NoEresElOwner,

    #[msg("Error, el zapato con el que deseas interactuar no existe")]
    ZapatoNoExiste,
}

//////////////////////////// Cuenta Zapateria /////////////////////////////////////

#[account]
#[derive(InitSpace)]
pub struct Zapateria {

    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(20)]
    zapatos: Vec<Zapato>,
}

//////////////////////////// Struct Zapato /////////////////////////////////////

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Zapato {

    #[max_len(60)]
    nombre: String,

    talla: u8,

    precio: u16,

    disponible: bool,
}

//////////////////////////// Contexto Crear Zapateria /////////////////////////////////////

#[derive(Accounts)]
pub struct NuevaZapateria<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Zapateria::INIT_SPACE + 8,
        seeds = [b"zapateria", owner.key().as_ref()],
        bump
    )]
    pub zapateria: Account<'info, Zapateria>,

    pub system_program: Program<'info, System>,
}

//////////////////////////// Contexto Zapatos /////////////////////////////////////

#[derive(Accounts)]
pub struct NuevoZapato<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub zapateria: Account<'info, Zapateria>,
}
