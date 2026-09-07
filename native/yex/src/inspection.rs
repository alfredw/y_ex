//! Experimental observation APIs only. Application schema policy stays in Elixir.
use crate::doc::DocOperations;
use crate::{atoms, NifDoc};
use rustler::{Atom, Binary, NifResult};
use serde_json::{json, Value};
use yrs::encoding::read::Cursor;
use yrs::updates::decoder::{Decode, Decoder, DecoderV1, DecoderV2};
use yrs::{DeleteSet, ReadTxn, Update};

fn deletions(set: &DeleteSet) -> Vec<Value> {
    set.iter()
        .flat_map(|(client, ranges)| {
            ranges
                .iter()
                .map(move |range| json!({"client": client, "start": range.start, "end": range.end}))
        })
        .collect()
}

#[rustler::nif(schedule = "DirtyCpu")]
fn inspect_update(update: Binary, v2: bool) -> NifResult<(Atom, String)> {
    if update.len() > 1_048_576 {
        return Err(rustler::Error::BadArg);
    }
    let decoded = if v2 {
        let mut decoder =
            DecoderV2::new(Cursor::new(update.as_slice())).map_err(|_| rustler::Error::BadArg)?;
        complete_update(&mut decoder)?
    } else {
        complete_update(&mut DecoderV1::from(update.as_slice()))?
    };
    Ok((
        atoms::ok(),
        json!({
            "version": 1, "blocks": decoded.inspect_blocks(),
            "deletions": deletions(decoded.delete_set())
        })
        .to_string(),
    ))
}

fn complete_update<D: Decoder>(decoder: &mut D) -> NifResult<Update> {
    let update = Update::decode(decoder).map_err(|_| rustler::Error::BadArg)?;
    if !decoder
        .read_to_end()
        .map_err(|_| rustler::Error::BadArg)?
        .is_empty()
    {
        return Err(rustler::Error::BadArg);
    }
    Ok(update)
}

#[rustler::nif(schedule = "DirtyCpu")]
fn inspect_document(doc: NifDoc) -> NifResult<(Atom, String)> {
    doc.with_transaction(|txn| {
        let mut roots: Vec<_> = txn.root_refs().map(|(name, _)| name.to_string()).collect();
        roots.sort();
        let root_types: Vec<_> = txn.root_refs().map(|(name, value)| {
            json!({"name": name, "observed_kind": value.try_branch().map(|branch| branch.type_ref().kind())})
        }).collect();
        let pending = txn.store().pending_update().map(|pending| {
            let missing: Vec<_> = pending.missing.iter().map(|(client, clock)| (*client, *clock)).collect();
            json!({"missing": missing, "blocks": pending.update.inspect_blocks()})
        });
        let pending_deletions = txn.store().pending_ds().map(deletions).unwrap_or_default();
        Ok((atoms::ok(), json!({
            "version": 1, "roots": roots, "root_types": root_types, "pending": pending, "pending_deletions": pending_deletions
        }).to_string()))
    })
}
