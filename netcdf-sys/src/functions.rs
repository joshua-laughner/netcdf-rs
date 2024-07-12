use std::os::raw::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulonglong, c_ushort, c_void,
};

// #[cfg(feature = "4.6.2")]
#[cfg(feature = "4.8.0")]
use super::NC_Dispatch;

pub type nc_type = c_int;

#[repr(C)]
pub struct nc_vlen_t {
    pub len: usize,
    pub p: *mut c_void,
}

extern "C" {
    pub fn nc_inq_libvers() -> *const c_char;
    pub fn nc_strerror(ncerr: c_int) -> *const c_char;

    // #[cfg(feature = "4.6.2")]
    #[cfg(feature = "4.8.0")]
    pub fn nc_def_user_format(
        mode_flag: c_int,
        dispatch_table: *mut NC_Dispatch,
        magic_number: *mut c_char,
    ) -> c_int;
    //#[cfg(feature = "4.6.2")]
    #[cfg(feature = "4.8.0")]
    pub fn nc_inq_user_format(
        mode_flag: c_int,
        dispatch_table: *mut *mut NC_Dispatch,
        magic_number: *mut c_char,
    ) -> c_int;

    pub fn nc__create(
        path: *const c_char,
        cmode: c_int,
        initialsz: usize,
        chunksizehintp: *mut usize,
        ncidp: *mut c_int,
    ) -> c_int;
    pub fn nc_create(path: *const c_char, cmode: c_int, ncidp: *mut c_int) -> c_int;
    pub fn nc__open(
        path: *const c_char,
        mode: c_int,
        chunksizehintp: *mut usize,
        ncidp: *mut c_int,
    ) -> c_int;
    pub fn nc_open(path: *const c_char, mode: c_int, ncidp: *mut c_int) -> c_int;
    pub fn nc_inq_path(ncid: c_int, pathlen: *mut usize, path: *mut c_char) -> c_int;
    pub fn nc_inq_ncid(ncid: c_int, name: *const c_char, grp_ncid: *mut c_int) -> c_int;
    pub fn nc_inq_grps(ncid: c_int, numgrps: *mut c_int, ncids: *mut c_int) -> c_int;
    pub fn nc_inq_grpname(ncid: c_int, name: *mut c_char) -> c_int;
    pub fn nc_inq_grpname_full(ncid: c_int, lenp: *mut usize, full_name: *mut c_char) -> c_int;
    pub fn nc_inq_grpname_len(ncid: c_int, lenp: *mut usize) -> c_int;
    pub fn nc_inq_grp_parent(ncid: c_int, parent_ncid: *mut c_int) -> c_int;
    pub fn nc_inq_grp_ncid(ncid: c_int, grp_name: *const c_char, grp_ncid: *mut c_int) -> c_int;
    pub fn nc_inq_grp_full_ncid(
        ncid: c_int,
        full_name: *const c_char,
        grp_ncid: *mut c_int,
    ) -> c_int;
    pub fn nc_inq_varids(ncid: c_int, nvars: *mut c_int, varids: *mut c_int) -> c_int;
    pub fn nc_inq_dimids(
        ncid: c_int,
        ndims: *mut c_int,
        dimids: *mut c_int,
        include_parents: c_int,
    ) -> c_int;
    pub fn nc_inq_typeids(ncid: c_int, ntypes: *mut c_int, typeids: *mut c_int) -> c_int;
    pub fn nc_inq_type_equal(
        ncid1: c_int,
        typeid1: nc_type,
        ncid2: c_int,
        typeid2: nc_type,
        equal: *mut c_int,
    ) -> c_int;
    pub fn nc_def_grp(parent_ncid: c_int, name: *const c_char, new_ncid: *mut c_int) -> c_int;
    pub fn nc_rename_grp(grpid: c_int, name: *const c_char) -> c_int;
    pub fn nc_def_compound(
        ncid: c_int,
        size: usize,
        name: *const c_char,
        typeidp: *mut nc_type,
    ) -> c_int;
    pub fn nc_insert_compound(
        ncid: c_int,
        xtype: nc_type,
        name: *const c_char,
        offset: usize,
        field_typeid: nc_type,
    ) -> c_int;
    pub fn nc_insert_array_compound(
        ncid: c_int,
        xtype: nc_type,
        name: *const c_char,
        offset: usize,
        field_typeid: nc_type,
        ndims: c_int,
        dim_sizes: *const c_int,
    ) -> c_int;
    pub fn nc_inq_type(ncid: c_int, xtype: nc_type, name: *mut c_char, size: *mut usize) -> c_int;
    pub fn nc_inq_typeid(ncid: c_int, name: *const c_char, typeidp: *mut nc_type) -> c_int;
    pub fn nc_inq_compound(
        ncid: c_int,
        xtype: nc_type,
        name: *mut c_char,
        sizep: *mut usize,
        nfieldsp: *mut usize,
    ) -> c_int;
    pub fn nc_inq_compound_name(ncid: c_int, xtype: nc_type, name: *mut c_char) -> c_int;
    pub fn nc_inq_compound_size(ncid: c_int, xtype: nc_type, sizep: *mut usize) -> c_int;
    pub fn nc_inq_compound_nfields(ncid: c_int, xtype: nc_type, nfieldsp: *mut usize) -> c_int;
    pub fn nc_inq_compound_field(
        ncid: c_int,
        xtype: nc_type,
        fieldid: c_int,
        name: *mut c_char,
        offsetp: *mut usize,
        field_typeidp: *mut nc_type,
        ndimsp: *mut c_int,
        dim_sizesp: *mut c_int,
    ) -> c_int;
    pub fn nc_inq_compound_fieldname(
        ncid: c_int,
        xtype: nc_type,
        fieldid: c_int,
        name: *mut c_char,
    ) -> c_int;
    pub fn nc_inq_compound_fieldindex(
        ncid: c_int,
        xtype: nc_type,
        name: *const c_char,
        fieldidp: *mut c_int,
    ) -> c_int;
    pub fn nc_inq_compound_fieldoffset(
        ncid: c_int,
        xtype: nc_type,
        fieldid: c_int,
        offsetp: *mut usize,
    ) -> c_int;
    pub fn nc_inq_compound_fieldtype(
        ncid: c_int,
        xtype: nc_type,
        fieldid: c_int,
        field_typeidp: *mut nc_type,
    ) -> c_int;
    pub fn nc_inq_compound_fieldndims(
        ncid: c_int,
        xtype: nc_type,
        fieldid: c_int,
        ndimsp: *mut c_int,
    ) -> c_int;
    pub fn nc_inq_compound_fielddim_sizes(
        ncid: c_int,
        xtype: nc_type,
        fieldid: c_int,
        dim_sizes: *mut c_int,
    ) -> c_int;
    pub fn nc_def_vlen(
        ncid: c_int,
        name: *const c_char,
        base_typeid: nc_type,
        xtypep: *mut nc_type,
    ) -> c_int;
    pub fn nc_inq_vlen(
        ncid: c_int,
        xtype: nc_type,
        name: *mut c_char,
        datum_sizep: *mut usize,
        base_nc_typep: *mut nc_type,
    ) -> c_int;
    pub fn nc_free_vlen(vl: *mut nc_vlen_t) -> c_int;
    pub fn nc_free_vlens(len: usize, vlens: *mut nc_vlen_t) -> c_int;
    pub fn nc_put_vlen_element(
        ncid: c_int,
        typeid1: c_int,
        vlen_element: *mut c_void,
        len: usize,
        data: *const c_void,
    ) -> c_int;
    pub fn nc_get_vlen_element(
        ncid: c_int,
        typeid1: c_int,
        vlen_element: *const c_void,
        len: *mut usize,
        data: *mut c_void,
    ) -> c_int;
    pub fn nc_free_string(len: usize, data: *mut *mut c_char) -> c_int;
    pub fn nc_inq_user_type(
        ncid: c_int,
        xtype: nc_type,
        name: *mut c_char,
        size: *mut usize,
        base_nc_typep: *mut nc_type,
        nfieldsp: *mut usize,
        classp: *mut c_int,
    ) -> c_int;
    pub fn nc_put_att(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_void,
    ) -> c_int;
    pub fn nc_get_att(ncid: c_int, varid: c_int, name: *const c_char, ip: *mut c_void) -> c_int;
    pub fn nc_def_enum(
        ncid: c_int,
        base_typeid: nc_type,
        name: *const c_char,
        typeidp: *mut nc_type,
    ) -> c_int;
    pub fn nc_insert_enum(
        ncid: c_int,
        xtype: nc_type,
        name: *const c_char,
        value: *const c_void,
    ) -> c_int;
    pub fn nc_inq_enum(
        ncid: c_int,
        xtype: nc_type,
        name: *mut c_char,
        base_nc_typep: *mut nc_type,
        base_sizep: *mut usize,
        num_membersp: *mut usize,
    ) -> c_int;
    pub fn nc_inq_enum_member(
        ncid: c_int,
        xtype: nc_type,
        idx: c_int,
        name: *mut c_char,
        value: *mut c_void,
    ) -> c_int;
    pub fn nc_inq_enum_ident(
        ncid: c_int,
        xtype: nc_type,
        value: c_longlong,
        identifier: *mut c_char,
    ) -> c_int;
    pub fn nc_def_opaque(
        ncid: c_int,
        size: usize,
        name: *const c_char,
        xtypep: *mut nc_type,
    ) -> c_int;
    pub fn nc_inq_opaque(
        ncid: c_int,
        xtype: nc_type,
        name: *mut c_char,
        sizep: *mut usize,
    ) -> c_int;
    pub fn nc_put_var(ncid: c_int, varid: c_int, op: *const c_void) -> c_int;
    pub fn nc_get_var(ncid: c_int, varid: c_int, ip: *mut c_void) -> c_int;
    pub fn nc_put_var1(ncid: c_int, varid: c_int, indexp: *const usize, op: *const c_void)
        -> c_int;
    pub fn nc_get_var1(ncid: c_int, varid: c_int, indexp: *const usize, ip: *mut c_void) -> c_int;
    pub fn nc_put_vara(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_void,
    ) -> c_int;
    pub fn nc_get_vara(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_void,
    ) -> c_int;
    pub fn nc_put_vars(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_void,
    ) -> c_int;
    pub fn nc_get_vars(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_void,
    ) -> c_int;
    pub fn nc_put_varm(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_void,
    ) -> c_int;
    pub fn nc_get_varm(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_void,
    ) -> c_int;
    pub fn nc_def_var_deflate(
        ncid: c_int,
        varid: c_int,
        shuffle: c_int,
        deflate: c_int,
        deflate_level: c_int,
    ) -> c_int;
    pub fn nc_inq_var_deflate(
        ncid: c_int,
        varid: c_int,
        shufflep: *mut c_int,
        deflatep: *mut c_int,
        deflate_levelp: *mut c_int,
    ) -> c_int;
    #[cfg(feature = "4.7.4")]
    pub fn nc_def_var_szip(
        ncid: c_int,
        varid: c_int,
        options_mask: c_int,
        pixels_per_block: c_int,
    ) -> c_int;
    pub fn nc_inq_var_szip(
        ncid: c_int,
        varid: c_int,
        options_maskp: *mut c_int,
        pixels_per_blockp: *mut c_int,
    ) -> c_int;
    pub fn nc_def_var_fletcher32(ncid: c_int, varid: c_int, fletcher32: c_int) -> c_int;
    pub fn nc_inq_var_fletcher32(ncid: c_int, varid: c_int, fletcher32p: *mut c_int) -> c_int;
    pub fn nc_def_var_chunking(
        ncid: c_int,
        varid: c_int,
        storage: c_int,
        chunksizesp: *const usize,
    ) -> c_int;
    pub fn nc_inq_var_chunking(
        ncid: c_int,
        varid: c_int,
        storagep: *mut c_int,
        chunksizesp: *mut usize,
    ) -> c_int;
    pub fn nc_def_var_fill(
        ncid: c_int,
        varid: c_int,
        no_fill: c_int,
        fill_value: *const c_void,
    ) -> c_int;
    pub fn nc_inq_var_fill(
        ncid: c_int,
        varid: c_int,
        no_fill: *mut c_int,
        fill_valuep: *mut c_void,
    ) -> c_int;
    pub fn nc_def_var_endian(ncid: c_int, varid: c_int, endian: c_int) -> c_int;
    pub fn nc_inq_var_endian(ncid: c_int, varid: c_int, endianp: *mut c_int) -> c_int;
    #[cfg(feature = "4.6.0")]
    pub fn nc_def_var_filter(
        ncid: c_int,
        varid: c_int,
        id: c_uint,
        nparams: usize,
        parms: *const c_uint,
    ) -> c_int;
    #[cfg(feature = "4.6.0")]
    pub fn nc_inq_var_filter(
        ncid: c_int,
        varid: c_int,
        idp: *const c_uint,
        nparams: *mut usize,
        params: *mut c_uint,
    ) -> c_int;
    pub fn nc_set_fill(ncid: c_int, fillmode: c_int, old_modep: *mut c_int) -> c_int;
    pub fn nc_set_default_format(format: c_int, old_formatp: *mut c_int) -> c_int;
    pub fn nc_set_chunk_cache(size: usize, nelems: usize, preemption: c_float) -> c_int;
    pub fn nc_get_chunk_cache(
        sizep: *mut usize,
        nelemsp: *mut usize,
        preemptionp: *mut c_float,
    ) -> c_int;
    pub fn nc_set_var_chunk_cache(
        ncid: c_int,
        varid: c_int,
        size: usize,
        nelems: usize,
        preemption: c_float,
    ) -> c_int;
    pub fn nc_get_var_chunk_cache(
        ncid: c_int,
        varid: c_int,
        sizep: *mut usize,
        nelemsp: *mut usize,
        preemptionp: *mut c_float,
    ) -> c_int;
    pub fn nc_redef(ncid: c_int) -> c_int;
    pub fn nc__enddef(
        ncid: c_int,
        h_minfree: usize,
        v_align: usize,
        v_minfree: usize,
        r_align: usize,
    ) -> c_int;
    pub fn nc_enddef(ncid: c_int) -> c_int;
    pub fn nc_sync(ncid: c_int) -> c_int;
    pub fn nc_abort(ncid: c_int) -> c_int;
    pub fn nc_close(ncid: c_int) -> c_int;
    pub fn nc_inq(
        ncid: c_int,
        ndimsp: *mut c_int,
        nvarsp: *mut c_int,
        nattsp: *mut c_int,
        unlimdimidp: *mut c_int,
    ) -> c_int;
    pub fn nc_inq_ndims(ncid: c_int, ndimsp: *mut c_int) -> c_int;
    pub fn nc_inq_nvars(ncid: c_int, nvarsp: *mut c_int) -> c_int;
    pub fn nc_inq_natts(ncid: c_int, nattsp: *mut c_int) -> c_int;
    pub fn nc_inq_unlimdim(ncid: c_int, unlimdimidp: *mut c_int) -> c_int;
    pub fn nc_inq_unlimdims(
        ncid: c_int,
        nunlimdimsp: *mut c_int,
        unlimdimidsp: *mut c_int,
    ) -> c_int;
    pub fn nc_inq_format(ncid: c_int, formatp: *mut c_int) -> c_int;
    pub fn nc_inq_format_extended(ncid: c_int, formatp: *mut c_int, modep: *mut c_int) -> c_int;
    pub fn nc_def_dim(ncid: c_int, name: *const c_char, len: usize, idp: *mut c_int) -> c_int;
    pub fn nc_inq_dimid(ncid: c_int, name: *const c_char, idp: *mut c_int) -> c_int;
    pub fn nc_inq_dim(ncid: c_int, dimid: c_int, name: *mut c_char, lenp: *mut usize) -> c_int;
    pub fn nc_inq_dimname(ncid: c_int, dimid: c_int, name: *mut c_char) -> c_int;
    pub fn nc_inq_dimlen(ncid: c_int, dimid: c_int, lenp: *mut usize) -> c_int;
    pub fn nc_rename_dim(ncid: c_int, dimid: c_int, name: *const c_char) -> c_int;
    pub fn nc_inq_att(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtypep: *mut nc_type,
        lenp: *mut usize,
    ) -> c_int;
    pub fn nc_inq_attid(ncid: c_int, varid: c_int, name: *const c_char, idp: *mut c_int) -> c_int;
    pub fn nc_inq_atttype(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtypep: *mut nc_type,
    ) -> c_int;
    pub fn nc_inq_attlen(ncid: c_int, varid: c_int, name: *const c_char, lenp: *mut usize)
        -> c_int;
    pub fn nc_inq_attname(ncid: c_int, varid: c_int, attnum: c_int, name: *mut c_char) -> c_int;
    pub fn nc_copy_att(
        ncid_in: c_int,
        varid_in: c_int,
        name: *const c_char,
        ncid_out: c_int,
        varid_out: c_int,
    ) -> c_int;
    pub fn nc_rename_att(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        newname: *const c_char,
    ) -> c_int;
    pub fn nc_del_att(ncid: c_int, varid: c_int, name: *const c_char) -> c_int;
    pub fn nc_put_att_text(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        len: usize,
        op: *const c_char,
    ) -> c_int;
    pub fn nc_get_att_text(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_char,
    ) -> c_int;
    pub fn nc_put_att_uchar(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_uchar,
    ) -> c_int;
    pub fn nc_get_att_uchar(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_uchar,
    ) -> c_int;
    pub fn nc_put_att_schar(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_schar,
    ) -> c_int;
    pub fn nc_get_att_schar(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_schar,
    ) -> c_int;
    pub fn nc_put_att_short(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_short,
    ) -> c_int;
    pub fn nc_get_att_short(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_short,
    ) -> c_int;
    pub fn nc_put_att_int(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_int,
    ) -> c_int;
    pub fn nc_get_att_int(ncid: c_int, varid: c_int, name: *const c_char, ip: *mut c_int) -> c_int;
    pub fn nc_put_att_long(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_long,
    ) -> c_int;
    pub fn nc_get_att_long(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_long,
    ) -> c_int;
    pub fn nc_put_att_float(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_float,
    ) -> c_int;
    pub fn nc_get_att_float(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_float,
    ) -> c_int;
    pub fn nc_put_att_double(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_double,
    ) -> c_int;
    pub fn nc_get_att_double(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_double,
    ) -> c_int;
    pub fn nc_put_att_ushort(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_ushort,
    ) -> c_int;
    pub fn nc_get_att_ushort(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_ushort,
    ) -> c_int;
    pub fn nc_put_att_uint(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_uint,
    ) -> c_int;
    pub fn nc_get_att_uint(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_uint,
    ) -> c_int;
    pub fn nc_put_att_longlong(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_longlong,
    ) -> c_int;
    pub fn nc_get_att_longlong(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_longlong,
    ) -> c_int;
    pub fn nc_put_att_ulonglong(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_ulonglong,
    ) -> c_int;
    pub fn nc_get_att_ulonglong(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_ulonglong,
    ) -> c_int;
    pub fn nc_put_att_string(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        len: usize,
        op: *mut *const c_char,
    ) -> c_int;
    pub fn nc_get_att_string(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut *mut c_char,
    ) -> c_int;
    pub fn nc_def_var(
        ncid: c_int,
        name: *const c_char,
        xtype: nc_type,
        ndims: c_int,
        dimidsp: *const c_int,
        varidp: *mut c_int,
    ) -> c_int;
    pub fn nc_inq_var(
        ncid: c_int,
        varid: c_int,
        name: *mut c_char,
        xtypep: *mut nc_type,
        ndimsp: *mut c_int,
        dimidsp: *mut c_int,
        nattsp: *mut c_int,
    ) -> c_int;
    pub fn nc_inq_varid(ncid: c_int, name: *const c_char, varidp: *mut c_int) -> c_int;
    pub fn nc_inq_varname(ncid: c_int, varid: c_int, name: *mut c_char) -> c_int;
    pub fn nc_inq_vartype(ncid: c_int, varid: c_int, xtypep: *mut nc_type) -> c_int;
    pub fn nc_inq_varndims(ncid: c_int, varid: c_int, ndimsp: *mut c_int) -> c_int;
    pub fn nc_inq_vardimid(ncid: c_int, varid: c_int, dimidsp: *mut c_int) -> c_int;
    pub fn nc_inq_varnatts(ncid: c_int, varid: c_int, nattsp: *mut c_int) -> c_int;
    pub fn nc_rename_var(ncid: c_int, varid: c_int, name: *const c_char) -> c_int;
    pub fn nc_copy_var(ncid_in: c_int, varid: c_int, ncid_out: c_int) -> c_int;
    pub fn nc_put_var1_text(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_char,
    ) -> c_int;
    pub fn nc_get_var1_text(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_char,
    ) -> c_int;
    pub fn nc_put_var1_uchar(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_uchar,
    ) -> c_int;
    pub fn nc_get_var1_uchar(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_uchar,
    ) -> c_int;
    pub fn nc_put_var1_schar(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_schar,
    ) -> c_int;
    pub fn nc_get_var1_schar(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_schar,
    ) -> c_int;
    pub fn nc_put_var1_short(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_short,
    ) -> c_int;
    pub fn nc_get_var1_short(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_short,
    ) -> c_int;
    pub fn nc_put_var1_int(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_int,
    ) -> c_int;
    pub fn nc_get_var1_int(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_int,
    ) -> c_int;
    pub fn nc_put_var1_long(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_long,
    ) -> c_int;
    pub fn nc_get_var1_long(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_long,
    ) -> c_int;
    pub fn nc_put_var1_float(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_float,
    ) -> c_int;
    pub fn nc_get_var1_float(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_float,
    ) -> c_int;
    pub fn nc_put_var1_double(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_double,
    ) -> c_int;
    pub fn nc_get_var1_double(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_double,
    ) -> c_int;
    pub fn nc_put_var1_ushort(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_ushort,
    ) -> c_int;
    pub fn nc_get_var1_ushort(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_ushort,
    ) -> c_int;
    pub fn nc_put_var1_uint(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_uint,
    ) -> c_int;
    pub fn nc_get_var1_uint(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_uint,
    ) -> c_int;
    pub fn nc_put_var1_longlong(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_longlong,
    ) -> c_int;
    pub fn nc_get_var1_longlong(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_longlong,
    ) -> c_int;
    pub fn nc_put_var1_ulonglong(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_ulonglong,
    ) -> c_int;
    pub fn nc_get_var1_ulonglong(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_ulonglong,
    ) -> c_int;
    pub fn nc_put_var1_string(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *mut *const c_char,
    ) -> c_int;
    pub fn nc_get_var1_string(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut *mut c_char,
    ) -> c_int;
    pub fn nc_put_vara_text(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_char,
    ) -> c_int;
    pub fn nc_get_vara_text(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_char,
    ) -> c_int;
    pub fn nc_put_vara_uchar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_uchar,
    ) -> c_int;
    pub fn nc_get_vara_uchar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_uchar,
    ) -> c_int;
    pub fn nc_put_vara_schar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_schar,
    ) -> c_int;
    pub fn nc_get_vara_schar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_schar,
    ) -> c_int;
    pub fn nc_put_vara_short(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_short,
    ) -> c_int;
    pub fn nc_get_vara_short(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_short,
    ) -> c_int;
    pub fn nc_put_vara_int(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_int,
    ) -> c_int;
    pub fn nc_get_vara_int(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_int,
    ) -> c_int;
    pub fn nc_put_vara_long(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_long,
    ) -> c_int;
    pub fn nc_get_vara_long(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_long,
    ) -> c_int;
    pub fn nc_put_vara_float(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_float,
    ) -> c_int;
    pub fn nc_get_vara_float(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_float,
    ) -> c_int;
    pub fn nc_put_vara_double(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_double,
    ) -> c_int;
    pub fn nc_get_vara_double(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_double,
    ) -> c_int;
    pub fn nc_put_vara_ushort(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_ushort,
    ) -> c_int;
    pub fn nc_get_vara_ushort(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_ushort,
    ) -> c_int;
    pub fn nc_put_vara_uint(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_uint,
    ) -> c_int;
    pub fn nc_get_vara_uint(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_uint,
    ) -> c_int;
    pub fn nc_put_vara_longlong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_longlong,
    ) -> c_int;
    pub fn nc_get_vara_longlong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_longlong,
    ) -> c_int;
    pub fn nc_put_vara_ulonglong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_ulonglong,
    ) -> c_int;
    pub fn nc_get_vara_ulonglong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_ulonglong,
    ) -> c_int;
    pub fn nc_put_vara_string(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *mut *const c_char,
    ) -> c_int;
    pub fn nc_get_vara_string(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut *mut c_char,
    ) -> c_int;
    pub fn nc_put_vars_text(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_char,
    ) -> c_int;
    pub fn nc_get_vars_text(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_char,
    ) -> c_int;
    pub fn nc_put_vars_uchar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_uchar,
    ) -> c_int;
    pub fn nc_get_vars_uchar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_uchar,
    ) -> c_int;
    pub fn nc_put_vars_schar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_schar,
    ) -> c_int;
    pub fn nc_get_vars_schar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_schar,
    ) -> c_int;
    pub fn nc_put_vars_short(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_short,
    ) -> c_int;
    pub fn nc_get_vars_short(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_short,
    ) -> c_int;
    pub fn nc_put_vars_int(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_int,
    ) -> c_int;
    pub fn nc_get_vars_int(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_int,
    ) -> c_int;
    pub fn nc_put_vars_long(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_long,
    ) -> c_int;
    pub fn nc_get_vars_long(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_long,
    ) -> c_int;
    pub fn nc_put_vars_float(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_float,
    ) -> c_int;
    pub fn nc_get_vars_float(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_float,
    ) -> c_int;
    pub fn nc_put_vars_double(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_double,
    ) -> c_int;
    pub fn nc_get_vars_double(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_double,
    ) -> c_int;
    pub fn nc_put_vars_ushort(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_ushort,
    ) -> c_int;
    pub fn nc_get_vars_ushort(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_ushort,
    ) -> c_int;
    pub fn nc_put_vars_uint(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_uint,
    ) -> c_int;
    pub fn nc_get_vars_uint(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_uint,
    ) -> c_int;
    pub fn nc_put_vars_longlong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_longlong,
    ) -> c_int;
    pub fn nc_get_vars_longlong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_longlong,
    ) -> c_int;
    pub fn nc_put_vars_ulonglong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_ulonglong,
    ) -> c_int;
    pub fn nc_get_vars_ulonglong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_ulonglong,
    ) -> c_int;
    pub fn nc_put_vars_string(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *mut *const c_char,
    ) -> c_int;
    pub fn nc_get_vars_string(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut *mut c_char,
    ) -> c_int;
    pub fn nc_put_varm_text(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_char,
    ) -> c_int;
    pub fn nc_get_varm_text(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_char,
    ) -> c_int;
    pub fn nc_put_varm_uchar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_uchar,
    ) -> c_int;
    pub fn nc_get_varm_uchar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_uchar,
    ) -> c_int;
    pub fn nc_put_varm_schar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_schar,
    ) -> c_int;
    pub fn nc_get_varm_schar(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_schar,
    ) -> c_int;
    pub fn nc_put_varm_short(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_short,
    ) -> c_int;
    pub fn nc_get_varm_short(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_short,
    ) -> c_int;
    pub fn nc_put_varm_int(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_int,
    ) -> c_int;
    pub fn nc_get_varm_int(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_int,
    ) -> c_int;
    pub fn nc_put_varm_long(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_long,
    ) -> c_int;
    pub fn nc_get_varm_long(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_long,
    ) -> c_int;
    pub fn nc_put_varm_float(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_float,
    ) -> c_int;
    pub fn nc_get_varm_float(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_float,
    ) -> c_int;
    pub fn nc_put_varm_double(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_double,
    ) -> c_int;
    pub fn nc_get_varm_double(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_double,
    ) -> c_int;
    pub fn nc_put_varm_ushort(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_ushort,
    ) -> c_int;
    pub fn nc_get_varm_ushort(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_ushort,
    ) -> c_int;
    pub fn nc_put_varm_uint(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_uint,
    ) -> c_int;
    pub fn nc_get_varm_uint(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_uint,
    ) -> c_int;
    pub fn nc_put_varm_longlong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_longlong,
    ) -> c_int;
    pub fn nc_get_varm_longlong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_longlong,
    ) -> c_int;
    pub fn nc_put_varm_ulonglong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_ulonglong,
    ) -> c_int;
    pub fn nc_get_varm_ulonglong(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_ulonglong,
    ) -> c_int;
    pub fn nc_put_varm_string(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *mut *const c_char,
    ) -> c_int;
    pub fn nc_get_varm_string(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut *mut c_char,
    ) -> c_int;
    pub fn nc_put_var_text(ncid: c_int, varid: c_int, op: *const c_char) -> c_int;
    pub fn nc_get_var_text(ncid: c_int, varid: c_int, ip: *mut c_char) -> c_int;
    pub fn nc_put_var_uchar(ncid: c_int, varid: c_int, op: *const c_uchar) -> c_int;
    pub fn nc_get_var_uchar(ncid: c_int, varid: c_int, ip: *mut c_uchar) -> c_int;
    pub fn nc_put_var_schar(ncid: c_int, varid: c_int, op: *const c_schar) -> c_int;
    pub fn nc_get_var_schar(ncid: c_int, varid: c_int, ip: *mut c_schar) -> c_int;
    pub fn nc_put_var_short(ncid: c_int, varid: c_int, op: *const c_short) -> c_int;
    pub fn nc_get_var_short(ncid: c_int, varid: c_int, ip: *mut c_short) -> c_int;
    pub fn nc_put_var_int(ncid: c_int, varid: c_int, op: *const c_int) -> c_int;
    pub fn nc_get_var_int(ncid: c_int, varid: c_int, ip: *mut c_int) -> c_int;
    pub fn nc_put_var_long(ncid: c_int, varid: c_int, op: *const c_long) -> c_int;
    pub fn nc_get_var_long(ncid: c_int, varid: c_int, ip: *mut c_long) -> c_int;
    pub fn nc_put_var_float(ncid: c_int, varid: c_int, op: *const c_float) -> c_int;
    pub fn nc_get_var_float(ncid: c_int, varid: c_int, ip: *mut c_float) -> c_int;
    pub fn nc_put_var_double(ncid: c_int, varid: c_int, op: *const c_double) -> c_int;
    pub fn nc_get_var_double(ncid: c_int, varid: c_int, ip: *mut c_double) -> c_int;
    pub fn nc_put_var_ushort(ncid: c_int, varid: c_int, op: *const c_ushort) -> c_int;
    pub fn nc_get_var_ushort(ncid: c_int, varid: c_int, ip: *mut c_ushort) -> c_int;
    pub fn nc_put_var_uint(ncid: c_int, varid: c_int, op: *const c_uint) -> c_int;
    pub fn nc_get_var_uint(ncid: c_int, varid: c_int, ip: *mut c_uint) -> c_int;
    pub fn nc_put_var_longlong(ncid: c_int, varid: c_int, op: *const c_longlong) -> c_int;
    pub fn nc_get_var_longlong(ncid: c_int, varid: c_int, ip: *mut c_longlong) -> c_int;
    pub fn nc_put_var_ulonglong(ncid: c_int, varid: c_int, op: *const c_ulonglong) -> c_int;
    pub fn nc_get_var_ulonglong(ncid: c_int, varid: c_int, ip: *mut c_ulonglong) -> c_int;
    pub fn nc_put_var_string(ncid: c_int, varid: c_int, op: *mut *const c_char) -> c_int;
    pub fn nc_get_var_string(ncid: c_int, varid: c_int, ip: *mut *mut c_char) -> c_int;
    pub fn nc_put_att_ubyte(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        xtype: nc_type,
        len: usize,
        op: *const c_uchar,
    ) -> c_int;
    pub fn nc_get_att_ubyte(
        ncid: c_int,
        varid: c_int,
        name: *const c_char,
        ip: *mut c_uchar,
    ) -> c_int;
    pub fn nc_put_var1_ubyte(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        op: *const c_uchar,
    ) -> c_int;
    pub fn nc_get_var1_ubyte(
        ncid: c_int,
        varid: c_int,
        indexp: *const usize,
        ip: *mut c_uchar,
    ) -> c_int;
    pub fn nc_put_vara_ubyte(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        op: *const c_uchar,
    ) -> c_int;
    pub fn nc_get_vara_ubyte(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        ip: *mut c_uchar,
    ) -> c_int;
    pub fn nc_put_vars_ubyte(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        op: *const c_uchar,
    ) -> c_int;
    pub fn nc_get_vars_ubyte(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        ip: *mut c_uchar,
    ) -> c_int;
    pub fn nc_put_varm_ubyte(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        op: *const c_uchar,
    ) -> c_int;
    pub fn nc_get_varm_ubyte(
        ncid: c_int,
        varid: c_int,
        startp: *const usize,
        countp: *const usize,
        stridep: *const isize,
        imapp: *const isize,
        ip: *mut c_uchar,
    ) -> c_int;
    pub fn nc_put_var_ubyte(ncid: c_int, varid: c_int, op: *const c_uchar) -> c_int;
    pub fn nc_get_var_ubyte(ncid: c_int, varid: c_int, ip: *mut c_uchar) -> c_int;

    // #[cfg(feature = "")]
    pub fn nc_set_log_level(new_level: c_int) -> c_int;

    pub fn nc_show_metadata(ncid: c_int) -> c_int;

    pub fn nc_delete(path: *const c_char) -> c_int;

    pub fn nc__create_mp(
        path: *const c_char,
        cmode: c_int,
        initialsz: usize,
        basepe: c_int,
        chunksizehintp: *mut usize,
        ncidp: *mut c_int,
    ) -> c_int;
    pub fn nc__open_mp(
        path: *const c_char,
        mode: c_int,
        basepe: c_int,
        chunksizehintp: *mut usize,
        ncidp: *mut c_int,
    ) -> c_int;
    pub fn nc_delete_mp(path: *const c_char, basepe: c_int) -> c_int;
    pub fn nc_set_base_pe(ncid: c_int, pe: c_int) -> c_int;
    pub fn nc_inq_base_pe(ncid: c_int, pe: *mut c_int) -> c_int;
    pub fn nctypelen(datatype: nc_type) -> c_int;
}

/// Backward compatability
mod netcdf_2 {
    use super::*;

    pub type nclong = c_int;

    extern "C" {
        //#[cfg(feature = "..")]
        pub static mut ncerr: c_int;
        //#[cfg(feature = "..")]
        pub static mut ncopts: c_int;

        pub fn nc_advise(cdf_routine_name: *const c_char, err: c_int, fmt: *const c_char, ...);
        pub fn nccreate(path: *const c_char, cmode: c_int) -> c_int;
        pub fn ncopen(path: *const c_char, mode: c_int) -> c_int;
        pub fn ncsetfill(ncid: c_int, fillmode: c_int) -> c_int;
        pub fn ncredef(ncid: c_int) -> c_int;
        pub fn ncendef(ncid: c_int) -> c_int;
        pub fn ncsync(ncid: c_int) -> c_int;
        pub fn ncabort(ncid: c_int) -> c_int;
        pub fn ncclose(ncid: c_int) -> c_int;
        pub fn ncinquire(
            ncid: c_int,
            ndimsp: *mut c_int,
            nvarsp: *mut c_int,
            nattsp: *mut c_int,
            unlimdimp: *mut c_int,
        ) -> c_int;
        pub fn ncdimdef(ncid: c_int, name: *const c_char, len: c_long) -> c_int;
        pub fn ncdimid(ncid: c_int, name: *const c_char) -> c_int;
        pub fn ncdiminq(ncid: c_int, dimid: c_int, name: *mut c_char, lenp: *mut c_long) -> c_int;
        pub fn ncdimrename(ncid: c_int, dimid: c_int, name: *const c_char) -> c_int;
        pub fn ncattput(
            ncid: c_int,
            varid: c_int,
            name: *const c_char,
            xtype: nc_type,
            len: c_int,
            op: *const c_void,
        ) -> c_int;
        pub fn ncattinq(
            ncid: c_int,
            varid: c_int,
            name: *const c_char,
            xtypep: *mut nc_type,
            lenp: *mut c_int,
        ) -> c_int;
        pub fn ncattget(ncid: c_int, varid: c_int, name: *const c_char, ip: *mut c_void) -> c_int;
        pub fn ncattcopy(
            ncid_in: c_int,
            varid_in: c_int,
            name: *const c_char,
            ncid_out: c_int,
            varid_out: c_int,
        ) -> c_int;
        pub fn ncattname(ncid: c_int, varid: c_int, attnum: c_int, name: *mut c_char) -> c_int;
        pub fn ncattrename(
            ncid: c_int,
            varid: c_int,
            name: *const c_char,
            newname: *const c_char,
        ) -> c_int;
        pub fn ncattdel(ncid: c_int, varid: c_int, name: *const c_char) -> c_int;
        pub fn ncvardef(
            ncid: c_int,
            name: *const c_char,
            xtype: nc_type,
            ndims: c_int,
            dimidsp: *const c_int,
        ) -> c_int;
        pub fn ncvarid(ncid: c_int, name: *const c_char) -> c_int;
        pub fn ncvarinq(
            ncid: c_int,
            varid: c_int,
            name: *mut c_char,
            xtypep: *mut nc_type,
            ndimsp: *mut c_int,
            dimidsp: *mut c_int,
            nattsp: *mut c_int,
        ) -> c_int;
        pub fn ncvarput1(
            ncid: c_int,
            varid: c_int,
            indexp: *const c_long,
            op: *const c_void,
        ) -> c_int;
        pub fn ncvarget1(
            ncid: c_int,
            varid: c_int,
            indexp: *const c_long,
            ip: *mut c_void,
        ) -> c_int;
        pub fn ncvarput(
            ncid: c_int,
            varid: c_int,
            startp: *const c_long,
            countp: *const c_long,
            op: *const c_void,
        ) -> c_int;
        pub fn ncvarget(
            ncid: c_int,
            varid: c_int,
            startp: *const c_long,
            countp: *const c_long,
            ip: *mut c_void,
        ) -> c_int;
        pub fn ncvarputs(
            ncid: c_int,
            varid: c_int,
            startp: *const c_long,
            countp: *const c_long,
            stridep: *const c_long,
            op: *const c_void,
        ) -> c_int;
        pub fn ncvargets(
            ncid: c_int,
            varid: c_int,
            startp: *const c_long,
            countp: *const c_long,
            stridep: *const c_long,
            ip: *mut c_void,
        ) -> c_int;
        pub fn ncvarputg(
            ncid: c_int,
            varid: c_int,
            startp: *const c_long,
            countp: *const c_long,
            stridep: *const c_long,
            imapp: *const c_long,
            op: *const c_void,
        ) -> c_int;
        pub fn ncvargetg(
            ncid: c_int,
            varid: c_int,
            startp: *const c_long,
            countp: *const c_long,
            stridep: *const c_long,
            imapp: *const c_long,
            ip: *mut c_void,
        ) -> c_int;
        pub fn ncvarrename(ncid: c_int, varid: c_int, name: *const c_char) -> c_int;
        pub fn ncrecinq(
            ncid: c_int,
            nrecvarsp: *mut c_int,
            recvaridsp: *mut c_int,
            recsizesp: *mut c_long,
        ) -> c_int;
        pub fn ncrecget(ncid: c_int, recnum: c_long, datap: *mut *mut c_void) -> c_int;
        pub fn ncrecput(ncid: c_int, recnum: c_long, datap: *const *mut c_void) -> c_int;
        #[cfg(feature = "4.6.2")]
        pub fn nc_initialize() -> c_int;
        pub fn nc_finalize() -> c_int;

        #[cfg(feature = "4.9.2")]
        pub fn nc_rc_get(key: *const c_char) -> *mut c_char;
        #[cfg(feature = "4.9.2")]
        pub fn nc_rc_set(key: *const c_char, value: *const c_char) -> c_int;
    }
}
pub use netcdf_2::*;
