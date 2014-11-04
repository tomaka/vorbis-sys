#![allow(non_snake_case)]

extern crate libc;
extern crate "ogg-sys" as ogg;

#[repr(C)]
pub struct vorbis_info {
    pub version: libc::c_int,
    pub channels: libc::c_int,
    pub rate: libc::c_long,
    pub bitrate_upper: libc::c_long,
    pub bitrate_nominal: libc::c_long,
    pub bitrate_lower: libc::c_long,
    pub bitrate_window: libc::c_long,
    pub codec_setup: *mut libc::c_void,
}

#[repr(C)]
pub struct vorbis_dsp_state {
    pub analysisp: libc::c_int,
    pub vi: *mut vorbis_info,

    pub pcm: *mut *mut libc::c_float,
    pub pcmret: *mut *mut libc::c_float,
    pub pcm_storage: libc::c_int,
    pub pcm_current: libc::c_int,
    pub pcm_returned: libc::c_int,

    pub preextrapolate: libc::c_int,
    pub eofflag: libc::c_int,

    pub lW: libc::c_long,
    pub W: libc::c_long,
    pub nW: libc::c_long,
    pub centerW: libc::c_long,

    pub granulepos: ogg::ogg_int64_t,
    pub sequence: ogg::ogg_int64_t,

    pub glue_bits: ogg::ogg_int64_t,
    pub time_bits: ogg::ogg_int64_t,
    pub floor_bits: ogg::ogg_int64_t,
    pub res_bits: ogg::ogg_int64_t,

    pub backend_state: *mut libc::c_void,
}

#[repr(C)]
pub struct vorbis_block {
    pub pcm: *mut *mut libc::c_float,
    pub opb: ogg::oggpack_buffer,

    pub lW: libc::c_long,
    pub W: libc::c_long,
    pub nW: libc::c_long,
    pub pcmend: libc::c_int,
    pub mode: libc::c_int,

    pub eofflag: libc::c_int,
    pub granulepos: ogg::ogg_int64_t,
    pub sequence: ogg::ogg_int64_t,
    pub vd: *mut vorbis_dsp_state,

    pub localstore: *mut libc::c_void,
    pub localtop: libc::c_long,
    pub localalloc: libc::c_long,
    pub totaluse: libc::c_long,
    pub reap: *mut alloc_chain,

    pub glue_bits: libc::c_long,
    pub time_bits: libc::c_long,
    pub floor_bits: libc::c_long,
    pub res_bits: libc::c_long,

    pub internal: *mut libc::c_void,
}

#[repr(C)]
pub struct alloc_chain {
    pub ptr: *mut libc::c_char,
    pub next: *mut alloc_chain,
}

#[repr(C)]
pub struct vorbis_comment {
    pub user_comments: *mut *mut libc::c_char,
    pub comment_lengths: *mut libc::c_int,
    pub comments: libc::c_int,
    pub vendor: *mut libc::c_char,
}

pub const OV_FALSE: libc::c_int = -1;
pub const OV_EOF: libc::c_int = -2;
pub const OV_HOLE: libc::c_int = -3;

pub const OV_EREAD: libc::c_int = -128;
pub const OV_EFAULT: libc::c_int = -129;
pub const OV_EIMPL: libc::c_int = -130;
pub const OV_EINVAL: libc::c_int = -131;
pub const OV_ENOTVORBIS: libc::c_int = -132;
pub const OV_EBADHEADER: libc::c_int = -133;
pub const OV_EVERSION: libc::c_int = -134;
pub const OV_ENOTAUDIO: libc::c_int = -135;
pub const OV_EBADPACKET: libc::c_int = -136;
pub const OV_EBADLINK: libc::c_int = -137;
pub const OV_ENOSEEK: libc::c_int = -138;

extern {
    pub fn vorbis_info_init(vi: *mut vorbis_info);
    pub fn vorbis_info_clear(vi: *mut vorbis_info);
    pub fn vorbis_info_blocksize(vi: *mut vorbis_info, zo: libc::c_int) -> libc::c_int;
    pub fn vorbis_comment_init(vc: *mut vorbis_comment);
    pub fn vorbis_comment_add(vc: *mut vorbis_comment, comment: *const libc::c_char);
    pub fn vorbis_comment_add_tag(vc: *mut vorbis_comment, tag: *const libc::c_char,
        contents: *const libc::c_char);
    pub fn vorbis_comment_query(vc: *mut vorbis_comment, tag: *const libc::c_char,
        count: libc::c_int) -> *mut libc::c_char;
    pub fn vorbis_comment_query_count(vc: *mut vorbis_comment, tag: *const libc::c_char)
        -> libc::c_int;
    pub fn vorbis_comment_clear(vc: *mut vorbis_comment);

    pub fn vorbis_block_init(v: *mut vorbis_dsp_state, vb: *mut vorbis_block) -> libc::c_int;
    pub fn vorbis_block_clear(vb: *mut vorbis_block) -> libc::c_int;
    pub fn vorbis_dsp_clear(v: *mut vorbis_dsp_state);
    pub fn vorbis_granule_time(v: *mut vorbis_dsp_state, granulepos: ogg::ogg_int64_t)
        -> libc::c_double;

    pub fn vorbis_version_string() -> *const libc::c_char;

    pub fn vorbis_analysis_init(v: *mut vorbis_dsp_state,vi: *mut vorbis_info) -> libc::c_int;
    pub fn vorbis_commentheader_out(vc: *mut vorbis_comment, op: *mut ogg::ogg_packet)
        -> libc::c_int;
    pub fn vorbis_analysis_headerout(v: *mut vorbis_dsp_state, vc: *mut vorbis_comment,
        op: *mut ogg::ogg_packet, op_comm: *mut ogg::ogg_packet,
        op_code: *mut ogg::ogg_packet) -> libc::c_int;
    pub fn vorbis_analysis_buffer(v: *mut vorbis_dsp_state, vals: libc::c_int)
        -> *mut *mut libc::c_float;
    pub fn vorbis_analysis_wrote(v: *mut vorbis_dsp_state, vals: libc::c_int) -> libc::c_int;
    pub fn vorbis_analysis_blockout(v: *mut vorbis_dsp_state, vb: *mut vorbis_block) -> libc::c_int;
    pub fn vorbis_analysis(vb: *mut vorbis_block, op: *mut ogg::ogg_packet) -> libc::c_int;

    pub fn vorbis_bitrate_addblock(vb: *mut vorbis_block) -> libc::c_int;
    pub fn vorbis_bitrate_flushpacket(v: *mut vorbis_dsp_state, op: *mut ogg::ogg_packet)
        -> libc::c_int;

    pub fn vorbis_synthesis_idheader(op: *mut ogg::ogg_packet) -> libc::c_int;
    pub fn vorbis_synthesis_headerin(vi: *mut vorbis_info, vc: *mut vorbis_comment,
        op: *mut ogg::ogg_packet) -> libc::c_int;

    pub fn vorbis_synthesis_init(v: *mut vorbis_dsp_state, vi: *mut vorbis_info) -> libc::c_int;
    pub fn vorbis_synthesis_restart(v: *mut vorbis_dsp_state) -> libc::c_int;
    pub fn vorbis_synthesis(vb: *mut vorbis_block,op: *mut ogg::ogg_packet) -> libc::c_int;
    pub fn vorbis_synthesis_trackonly(vb: *mut vorbis_block,
        op: *mut ogg::ogg_packet) -> libc::c_int;
    pub fn vorbis_synthesis_blockin(v: *mut vorbis_dsp_state,vb: *mut vorbis_block) -> libc::c_int;
    pub fn vorbis_synthesis_pcmout(v: *mut vorbis_dsp_state, pcm: *mut *mut *mut libc::c_float)
        -> libc::c_int;
    pub fn vorbis_synthesis_lapout(v: *mut vorbis_dsp_state, pcm: *mut *mut *mut libc::c_float)
        -> libc::c_int;
    pub fn vorbis_synthesis_read(v: *mut vorbis_dsp_state, samples: libc::c_int) -> libc::c_int;
    pub fn vorbis_packet_blocksize(vi: *mut vorbis_info, op: *mut ogg::ogg_packet) -> libc::c_long;

    pub fn vorbis_synthesis_halfrate(v: *mut vorbis_info, flag: libc::c_int) -> libc::c_int;
    pub fn vorbis_synthesis_halfrate_p(v: *mut vorbis_info) -> libc::c_int;
}
