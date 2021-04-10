use crate::{
    misc::RuntimeResult,
    runtime::Runtime,
    types::{builtin::Builtin, list::List, t_object::TObject},
};

pub fn get_builtins() -> Vec<Builtin> {
    vec![
        Builtin::new("list", make_list),
        Builtin::new("cons", cons),
        Builtin::new("car", car),
        Builtin::new("cdr", cdr),
    ]
}

fn make_list(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    let evaled_args = args
        .iter()
        .map(|arg| ctx.eval(arg))
        .collect::<RuntimeResult<List>>()?;

    Ok(TObject::List(evaled_args))
}

fn cons(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [head, tail] => {
            let elements = match ctx.eval(tail)? {
                TObject::List(list) => Ok(list),
                _ => Err(format!("`cons` called with incorrect of args")),
            }?;

            let mut new_list = List::new();

            new_list.push_front(ctx.eval(head)?);

            for elem in elements.iter() {
                new_list.push_back(ctx.eval(elem)?)
            }
            Ok(TObject::List(new_list))
        }
        _ => Err(format!("`cons` called with incorrect args")),
    }
}

fn car(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [list] => match ctx.eval(list)? {
            TObject::List(list) => {
                let head = list
                    .get(0)
                    .ok_or(format!("Cannot get `car` of empty list"))?;
                Ok(head.clone())
            }
            _ => Err(format!("`car` called with incorrect args")),
        },
        _ => Err(format!("`car` called with incorrect args")),
    }
}

fn cdr(ctx: &mut Runtime, args: Vec<TObject>) -> RuntimeResult<TObject> {
    match &args[..] {
        [list] => match ctx.eval(list)? {
            TObject::List(list) => {
                let tail = list.iter().skip(1).map(TObject::clone).collect();
                Ok(TObject::List(tail))
            }
            _ => Err(format!("`car` called with incorrect args")),
        },
        _ => Err(format!("`car` called with incorrect args")),
    }
}
