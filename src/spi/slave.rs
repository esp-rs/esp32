#[doc = "Reader of register SLAVE"]
pub type R = crate::R<u32, super::SLAVE>;
#[doc = "Writer for register SLAVE"]
pub type W = crate::W<u32, super::SLAVE>;
#[doc = "Register SLAVE `reset()`'s with value 0"]
impl crate::ResetValue for super::SLAVE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYNC_RESET`"]
pub type SYNC_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYNC_RESET`"]
pub struct SYNC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_RESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SLAVE_MODE`"]
pub type SLAVE_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLAVE_MODE`"]
pub struct SLAVE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SLV_WR_RD_BUF_EN`"]
pub type SLV_WR_RD_BUF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_WR_RD_BUF_EN`"]
pub struct SLV_WR_RD_BUF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_RD_BUF_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `SLV_WR_RD_STA_EN`"]
pub type SLV_WR_RD_STA_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_WR_RD_STA_EN`"]
pub struct SLV_WR_RD_STA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_RD_STA_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `SLV_CMD_DEFINE`"]
pub type SLV_CMD_DEFINE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_CMD_DEFINE`"]
pub struct SLV_CMD_DEFINE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_CMD_DEFINE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Reader of field `TRANS_CNT`"]
pub type TRANS_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRANS_CNT`"]
pub struct TRANS_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | (((value as u32) & 0x0f) << 23);
        self.w
    }
}
#[doc = "Reader of field `SLV_LAST_STATE`"]
pub type SLV_LAST_STATE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLV_LAST_STATE`"]
pub struct SLV_LAST_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_LAST_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `SLV_LAST_COMMAND`"]
pub type SLV_LAST_COMMAND_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLV_LAST_COMMAND`"]
pub struct SLV_LAST_COMMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_LAST_COMMAND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | (((value as u32) & 0x07) << 17);
        self.w
    }
}
#[doc = "Reader of field `CS_I_MODE`"]
pub type CS_I_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CS_I_MODE`"]
pub struct CS_I_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_I_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `INT_EN`"]
pub type INT_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_EN`"]
pub struct INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
#[doc = "Reader of field `TRANS_DONE`"]
pub type TRANS_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TRANS_DONE`"]
pub struct TRANS_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SLV_WR_STA_DONE`"]
pub type SLV_WR_STA_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_WR_STA_DONE`"]
pub struct SLV_WR_STA_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_STA_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `SLV_RD_STA_DONE`"]
pub type SLV_RD_STA_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_RD_STA_DONE`"]
pub struct SLV_RD_STA_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_STA_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `SLV_WR_BUF_DONE`"]
pub type SLV_WR_BUF_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_WR_BUF_DONE`"]
pub struct SLV_WR_BUF_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_WR_BUF_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SLV_RD_BUF_DONE`"]
pub type SLV_RD_BUF_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_RD_BUF_DONE`"]
pub struct SLV_RD_BUF_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_RD_BUF_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Software reset enable, reset the spi clock line cs line and data lines."]
    #[inline(always)]
    pub fn sync_reset(&self) -> SYNC_RESET_R {
        SYNC_RESET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn slave_mode(&self) -> SLAVE_MODE_R {
        SLAVE_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - write and read buffer enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_buf_en(&self) -> SLV_WR_RD_BUF_EN_R {
        SLV_WR_RD_BUF_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - write and read status enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_sta_en(&self) -> SLV_WR_RD_STA_EN_R {
        SLV_WR_RD_STA_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as: 1: write-status 2: write-buffer and 3: read-buffer."]
    #[inline(always)]
    pub fn slv_cmd_define(&self) -> SLV_CMD_DEFINE_R {
        SLV_CMD_DEFINE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - The operations counter in both the master mode and the slave mode. 4: read-status"]
    #[inline(always)]
    pub fn trans_cnt(&self) -> TRANS_CNT_R {
        TRANS_CNT_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - In the slave mode it is the state of spi state machine."]
    #[inline(always)]
    pub fn slv_last_state(&self) -> SLV_LAST_STATE_R {
        SLV_LAST_STATE_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 17:19 - In the slave mode it is the value of command."]
    #[inline(always)]
    pub fn slv_last_command(&self) -> SLV_LAST_COMMAND_R {
        SLV_LAST_COMMAND_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 10:11 - In the slave mode this bits used to synchronize the input spi cs signal and eliminate spi cs jitter."]
    #[inline(always)]
    pub fn cs_i_mode(&self) -> CS_I_MODE_R {
        CS_I_MODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 5:9 - Interrupt enable bits for the below 5 sources"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode."]
    #[inline(always)]
    pub fn trans_done(&self) -> TRANS_DONE_R {
        TRANS_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for the completion of write-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_sta_done(&self) -> SLV_WR_STA_DONE_R {
        SLV_WR_STA_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for the completion of read-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_sta_done(&self) -> SLV_RD_STA_DONE_R {
        SLV_RD_STA_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for the completion of write-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&self) -> SLV_WR_BUF_DONE_R {
        SLV_WR_BUF_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The interrupt raw bit for the completion of read-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&self) -> SLV_RD_BUF_DONE_R {
        SLV_RD_BUF_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Software reset enable, reset the spi clock line cs line and data lines."]
    #[inline(always)]
    pub fn sync_reset(&mut self) -> SYNC_RESET_W {
        SYNC_RESET_W { w: self }
    }
    #[doc = "Bit 30 - 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn slave_mode(&mut self) -> SLAVE_MODE_W {
        SLAVE_MODE_W { w: self }
    }
    #[doc = "Bit 29 - write and read buffer enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_buf_en(&mut self) -> SLV_WR_RD_BUF_EN_W {
        SLV_WR_RD_BUF_EN_W { w: self }
    }
    #[doc = "Bit 28 - write and read status enable in the slave mode"]
    #[inline(always)]
    pub fn slv_wr_rd_sta_en(&mut self) -> SLV_WR_RD_STA_EN_W {
        SLV_WR_RD_STA_EN_W { w: self }
    }
    #[doc = "Bit 27 - 1: slave mode commands are defined in SPI_SLAVE3. 0: slave mode commands are fixed as: 1: write-status 2: write-buffer and 3: read-buffer."]
    #[inline(always)]
    pub fn slv_cmd_define(&mut self) -> SLV_CMD_DEFINE_W {
        SLV_CMD_DEFINE_W { w: self }
    }
    #[doc = "Bits 23:26 - The operations counter in both the master mode and the slave mode. 4: read-status"]
    #[inline(always)]
    pub fn trans_cnt(&mut self) -> TRANS_CNT_W {
        TRANS_CNT_W { w: self }
    }
    #[doc = "Bits 20:22 - In the slave mode it is the state of spi state machine."]
    #[inline(always)]
    pub fn slv_last_state(&mut self) -> SLV_LAST_STATE_W {
        SLV_LAST_STATE_W { w: self }
    }
    #[doc = "Bits 17:19 - In the slave mode it is the value of command."]
    #[inline(always)]
    pub fn slv_last_command(&mut self) -> SLV_LAST_COMMAND_W {
        SLV_LAST_COMMAND_W { w: self }
    }
    #[doc = "Bits 10:11 - In the slave mode this bits used to synchronize the input spi cs signal and eliminate spi cs jitter."]
    #[inline(always)]
    pub fn cs_i_mode(&mut self) -> CS_I_MODE_W {
        CS_I_MODE_W { w: self }
    }
    #[doc = "Bits 5:9 - Interrupt enable bits for the below 5 sources"]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W {
        INT_EN_W { w: self }
    }
    #[doc = "Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode."]
    #[inline(always)]
    pub fn trans_done(&mut self) -> TRANS_DONE_W {
        TRANS_DONE_W { w: self }
    }
    #[doc = "Bit 3 - The interrupt raw bit for the completion of write-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_sta_done(&mut self) -> SLV_WR_STA_DONE_W {
        SLV_WR_STA_DONE_W { w: self }
    }
    #[doc = "Bit 2 - The interrupt raw bit for the completion of read-status operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_sta_done(&mut self) -> SLV_RD_STA_DONE_W {
        SLV_RD_STA_DONE_W { w: self }
    }
    #[doc = "Bit 1 - The interrupt raw bit for the completion of write-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_wr_buf_done(&mut self) -> SLV_WR_BUF_DONE_W {
        SLV_WR_BUF_DONE_W { w: self }
    }
    #[doc = "Bit 0 - The interrupt raw bit for the completion of read-buffer operation in the slave mode."]
    #[inline(always)]
    pub fn slv_rd_buf_done(&mut self) -> SLV_RD_BUF_DONE_W {
        SLV_RD_BUF_DONE_W { w: self }
    }
}
