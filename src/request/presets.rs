#![allow(clippy::missing_panics_doc)]

use super::{
    ephemeris::{
        vectors::{TableFormat, VectorsBuilder},
        TimeSpec,
    },
    Body, Center, Query, QueryBuilder,
};

pub fn state_vectors<B: Into<Body>, C: Into<Center>>(
    target: B,
    center: C,
    time: TimeSpec,
) -> Query {
    let mut query = vectors(target, center, time);
    query.specific.table_format(TableFormat::State);
    query.build().unwrap()
}

pub fn position_vectors<B: Into<Body>, C: Into<Center>>(
    target: B,
    center: C,
    time: TimeSpec,
) -> Query {
    let mut query = vectors(target, center, time);
    query.specific.table_format(TableFormat::Position);
    query.build().unwrap()
}

pub fn velocity_vector<B: Into<Body>, C: Into<Center>>(
    target: B,
    center: C,
    time: TimeSpec,
) -> Query {
    let mut query = vectors(target, center, time);
    query.specific.table_format(TableFormat::Velocity);
    query.build().unwrap()
}

pub fn light_time_vectors<B: Into<Body>, C: Into<Center>>(
    target: B,
    center: C,
    time: TimeSpec,
) -> Query {
    let mut query = vectors(target, center, time);
    query.specific.table_format(TableFormat::LT);
    query.build().unwrap()
}

fn vectors<B: Into<Body>, C: Into<Center>>(
    target: B,
    center: C,
    time: TimeSpec,
) -> QueryBuilder<VectorsBuilder> {
    let mut query = Query::vectors();

    query.common.command(target).center(center).time_spec(time);

    query
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::{
        request::{
            bodies::MajorBody,
            ephemeris::TimeSpec,
            presets::{light_time_vectors, position_vectors, state_vectors, velocity_vector},
            Query,
        },
        TestResult,
    };

    #[test]
    fn test_vectors() -> TestResult {
        type VectorFn = fn(MajorBody, MajorBody, TimeSpec) -> Query;

        let funcs: [(VectorFn, &str); 4] = [
            (
                state_vectors,
                "command=502&ephem_type=V&center=500%40599\
            &ref_system=ICRF&format=text&obj_data=yes&make_ephem=yes\
            &csv_format=no&tlist=2022-08-31T00%3A00%3A00Z&vec_table=2\
            &vec_labels=yes&vec_delta_t=no&vec_corr=NONE&out_units=km-s&ref_plane=E",
            ),
            (
                velocity_vector,
                "command=502&ephem_type=V&center=500%40599\
            &ref_system=ICRF&format=text&obj_data=yes&make_ephem=yes\
            &csv_format=no&tlist=2022-08-31T00%3A00%3A00Z&vec_table=5\
            &vec_labels=yes&vec_delta_t=no&vec_corr=NONE&out_units=km-s&ref_plane=E",
            ),
            (
                position_vectors,
                "command=502&ephem_type=V&center=500%40599\
            &ref_system=ICRF&format=text&obj_data=yes&make_ephem=yes\
            &csv_format=no&tlist=2022-08-31T00%3A00%3A00Z&vec_table=1\
            &vec_labels=yes&vec_delta_t=no&vec_corr=NONE&out_units=km-s&ref_plane=E",
            ),
            (
                light_time_vectors,
                "command=502&ephem_type=V&center=500%40599\
            &ref_system=ICRF&format=text&obj_data=yes&make_ephem=yes\
            &csv_format=no&tlist=2022-08-31T00%3A00%3A00Z&vec_table=6\
            &vec_labels=yes&vec_delta_t=no&vec_corr=NONE&out_units=km-s&ref_plane=E",
            ),
        ];

        let (target, center, time) = (
            MajorBody::Europa,
            MajorBody::Jupiter,
            TimeSpec::from_list(vec![Utc.ymd(2022, 8, 31).and_hms(0, 0, 0)]),
        );

        for (func, expected) in funcs {
            let query = func(target, center, time.clone());

            assert_eq!(expected, serde_urlencoded::to_string(query)?);
        }

        Ok(())
    }
}
