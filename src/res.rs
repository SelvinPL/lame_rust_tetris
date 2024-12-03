#[macro_export]
macro_rules! count 
{
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

#[macro_export]
macro_rules! arr_from_consts 
{
    ($nv:vis const $ni:ident = [$( $(#[doc = $doc:expr])* $v:vis const $i:ident: $t:ty = $e:expr; )*]) => 
    {
        $(#[allow(dead_code)]
        $v const $i: $t = $e;)*
        #[allow(dead_code)]
        $nv const $ni: [&str; count!($($i)*)] = 
        [
            $(concat!(stringify!($i), "=", $e)),*
        ];
    }
}

arr_from_consts!(
    pub const RESOURCES =  [
        pub const IDR_MANIFEST: u32     = 1;
        pub const IDS_APP_TITLE: u32	= 101;
        pub const IDI_TETRIS: u32		= 102;
        pub const IDC_TETRIS: u32		= 103;
        pub const IDB_RECT: u32			= 104;
    ]
);
