use crate::*;

// helper method to read a given set of records of UnorderedMap
pub(crate) fn unordered_map_pagination<K, VV, V>(
    m: &UnorderedMap<K, VV>,
    from_index: Option<u64>,
    limit: Option<u64>,
) -> Vec<(K, V)>
    where
        K: BorshSerialize + BorshDeserialize,
        VV: BorshSerialize + BorshDeserialize,
        V: From<VV>,
{
    let keys = m.keys_as_vector();
    let values = m.values_as_vector();
    let from_index = from_index.unwrap_or(0);
    let limit = limit.unwrap_or(keys.len());
    (from_index..std::cmp::min(keys.len(), from_index + limit))
        .map(|index| (keys.get(index).unwrap(), values.get(index).unwrap().into()))
        .collect()
}

// format yNEAR value to human readable NEAR
pub(crate) fn format_ynear(value: u128) -> String {
    let value: f64 = (value / 1_000_000_000_000_000_000_000) as f64 / 1000f64;
    value.to_string()
}