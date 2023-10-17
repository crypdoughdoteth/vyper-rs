/// The vyper macro will take any arbitrary length sequence of two string literals (for the contract path + desired abi path)
/// The macro will parse the strings into a PathBuf and then into a Vyper struct. If there are
/// multiple pairs of paths, it will parse it into Vec<Vyper> instead.
#[macro_export]
macro_rules! vyper {
    ($p1: literal, $p2: literal) => {
        Vyper {path_to_code: PathBuf::from($p1), bytecode: None, abi: PathBuf::from($p2)}
    };
    ($($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            contracts
        }
    };
}

/// The vyper macro will take any arbitrary length sequence of two string literals (for the contract path + desired abi path)
/// The macro will call the vyper macro and push each newly constructed Vyper type to a Vector. From there, the Vector is type casted into the Vypers type and returned.
#[macro_export]
macro_rules! vypers{
    ($($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let cs: Vypers = contracts.into();
            cs
        }
    };
}

/// Instantiates the vyper struct using vyper! and compiles the contract too. Accepts two string
/// literals for the paths of the Vyper struct.
#[macro_export]
macro_rules! compile {
    // classic, simple
    ($p1: literal, $p2: literal) => {
        {
            let mut vy: Vyper = vyper!($p1, $p2);
            vy.compile().unwrap();
            vy
        }
    };
    // a twist on the classic: compile inside a venv with keyword venv
    (venv $p1: literal, $p2: literal) => {
        {
            let venv: Venv<Ready> = venv!();
            let mut vy: Vyper = vyper!($p1, $p2);
            venv.compile(&mut vy).unwrap();
            (vy, venv)
        }
    };
    // EVM Target: Paris
    (paris $p1: literal, $p2: literal) => {
        {
            let mut vy: Vyper = vyper!($p1, $p2);
            vy.compile_ver(Evm::Paris).unwrap();
            vy
        }
    };
    // compile to paris inside venv
    (venv paris $p1: literal, $p2: literal) => {
        {
            let venv: Venv<Ready> = venv!();
            let mut vy: Vyper = vyper!($p1, $p2);
            venv.compile_ver(&mut vy, Evm::Paris).unwrap();
            (vy, venv)
        }
    };
    // compile many
    ($($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many().await.unwrap();
            cs
        }
    };
    // compile many in venv
    (venv $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many(&mut cs).await.unwrap();
            (cs, venv)
        }
    };
   // Compile many for Paris
    (paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many_ver(Evm::Paris).await.unwrap();
            cs
        }
    };
    // compile many for paris inside venv
    (venv paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many_ver(&mut cs, Evm::Paris).await.unwrap();
            (cs, venv)
        }
    };
}

/// Instanitates the vyper struct and compiles the contract using compile! and generates an
/// abi file. Accepts two string literals for the paths of the Vyper struct.
#[macro_export]
macro_rules! abi {
    // OG matcher
    ($p1: literal, $p2: literal) => {
        {
            let c: Vyper = compile!($p1, $p2);
            c.abi().unwrap();
            c
        }
    };
    // OG matcher with a venv
    (venv $p1: literal, $p2: literal) => {
        {
            let (c, v): (Vyper, Venv<Ready>) = compile!(venv $p1, $p2);
            v.abi(&c).unwrap();
            (c, v)
        }
    };
    // return the ABI as json instead of creating a file
    (get $p1: literal, $p2: literal) => {
        {
            let c: Vyper = compile!($p1, $p2);
            let abi = c.abi_json().unwrap();
            (c, abi)
        }
    };
    // returns the ABI as JSON, Vypers struct, and Venv
    (venv get $p1: literal, $p2: literal) => {
        {
            let (c, v): (Vyper, Venv<Ready>) = compile!(venv $p1, $p2);
            let abi = v.abi_json(&c).unwrap();
            (c, abi, v)
        }
    };
    // gen abi, compile for paris
    (paris $p1: literal, $p2: literal) => {
        {
            let c: Vyper = compile!(paris $p1, $p2);
            c.abi().unwrap();
            c
        }
    };
    // return abi in json form, compile for paris
    (get paris $p1: literal, $p2: literal) => {
        {
            let c: Vyper = compile!(paris $p1, $p2);
            let abi = c.abi_json().unwrap();
            (c, abi)
        }
    };
    // gen abi, compile for paris
    (venv paris $p1: literal, $p2: literal) => {
        {
            let (c, v): (Vyper, Venv<Ready>) = compile!(venv paris $p1, $p2);
            v.abi(&c).unwrap();
            (c, v)
        }
    };
    // returns contract, abi in JSON form, venv
    (venv get paris $p1: literal, $p2: literal) => {
        {
            let (c, v): (Vyper, Venv<Ready>) = compile!(venv paris $p1, $p2);
            let abi = v.abi_json(&c).unwrap();
            (c, abi, v)
        }
    };

    // Generate many ABIs
    ($($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many().await.unwrap();
            cs.abi_many().await.unwrap();
            cs
        }
    };
    // return many ABIs as json
    (get $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many().await.unwrap();
            let abis = cs.abi_json_many().await.unwrap();
            (cs, abis)
        }
    };
    // venv version of many
    (venv $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many(&mut cs).await.unwrap();
            venv.abi_many(&cs).await.unwrap();
            (cs, venv)
        }
    };
    // return many ABIs w/ venv
    (venv get $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many(&mut cs).await.unwrap();
            let abi = venv.abi_json_many(&cs).await.unwrap();
            (cs, abi, venv)
        }
    };
    // gen abis compiled for Paris hard fork
    (paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many_ver(Evm::Paris).await.unwrap();
            cs.abi_many().await.unwrap();
            cs
        }
    };
    // gen abis compiled for Paris hard fork w/ venv
    (venv paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many_ver(&mut cs, Evm::Paris).await.unwrap();
            venv.abi_json_many(&cs).await.unwrap();
            (cs, venv)
        }
    };
    // return many abis compiled for paris
    (get paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            cs.compile_many_ver(Evm::Paris).await.unwrap();
            let abi = cs.abi_json_many().await.unwrap();
            (cs, abi)
        }
    };
    // return many abis compiled for paris w/ venv
    (venv get paris $($p1: literal, $p2: literal),+) => {
        {
            let mut contracts: Vec<Vyper> = vec![];
            $(
                let v = vyper!($p1, $p2);
                contracts.push(v);
            )+
            let mut cs: Vypers = contracts.into();
            let venv: Venv<Ready> = venv!();
            venv.compile_many_ver(&mut cs, Evm::Paris).await.unwrap();
            let abi = venv.abi_json_many(&cs).await.unwrap();
            (cs, abi, venv)
        }
    };
}

#[macro_export]
macro_rules! venv {
    () => {{
        Venv::new().init().unwrap().ivyper_venv(None).unwrap()
    }};
    ($ver: literal) => {{
        let version: &str = $ver;
        Venv::new()
            .init()
            .unwrap()
            .ivyper_venv(Some(version))
            .unwrap()
    }};
}
