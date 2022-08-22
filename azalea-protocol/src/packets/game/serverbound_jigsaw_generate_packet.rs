use azalea_buf::McBuf;
use azalea_core::BlockPos;
use packet_macros::ServerboundGamePacket;

#[derive(Clone, Debug, McBuf, ServerboundGamePacket)]
pub struct ServerboundJigsawGeneratePacket {
    pub pos: BlockPos,
    #[var]
    pub levels: u32,
    pub keep_jigsaws: bool,
}