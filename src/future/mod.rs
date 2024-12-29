moddef::moddef!(
    flat(pub) mod {
        actions_2d,
        actions,
        divide_and_conquer,
        reduce,
        runs,
        try_actions_2d,
        runs_2d,
        try_actions,
        try_runs,
        try_runs_2d
    },
    flat mod {
        maybe_done
    }
);

pub use slice_ops::future::*;