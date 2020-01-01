#[doc = "Reader of register LC_CONF"]
pub type R = crate::R<u32, super::LC_CONF>;
#[doc = "Writer for register LC_CONF"]
pub type W = crate::W<u32, super::LC_CONF>;
#[doc = "Register LC_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::LC_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_MEM_TRANS_EN`"]
pub type I2S_MEM_TRANS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_MEM_TRANS_EN`"]
pub struct I2S_MEM_TRANS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_MEM_TRANS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `I2S_CHECK_OWNER`"]
pub type I2S_CHECK_OWNER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_CHECK_OWNER`"]
pub struct I2S_CHECK_OWNER_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_CHECK_OWNER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `I2S_OUT_DATA_BURST_EN`"]
pub type I2S_OUT_DATA_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_OUT_DATA_BURST_EN`"]
pub struct I2S_OUT_DATA_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_OUT_DATA_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `I2S_INDSCR_BURST_EN`"]
pub type I2S_INDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_INDSCR_BURST_EN`"]
pub struct I2S_INDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_INDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `I2S_OUTDSCR_BURST_EN`"]
pub type I2S_OUTDSCR_BURST_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_OUTDSCR_BURST_EN`"]
pub struct I2S_OUTDSCR_BURST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_OUTDSCR_BURST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `I2S_OUT_EOF_MODE`"]
pub type I2S_OUT_EOF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_OUT_EOF_MODE`"]
pub struct I2S_OUT_EOF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_OUT_EOF_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2S_OUT_NO_RESTART_CLR`"]
pub type I2S_OUT_NO_RESTART_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_OUT_NO_RESTART_CLR`"]
pub struct I2S_OUT_NO_RESTART_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_OUT_NO_RESTART_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `I2S_OUT_AUTO_WRBACK`"]
pub type I2S_OUT_AUTO_WRBACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_OUT_AUTO_WRBACK`"]
pub struct I2S_OUT_AUTO_WRBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_OUT_AUTO_WRBACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `I2S_IN_LOOP_TEST`"]
pub type I2S_IN_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_IN_LOOP_TEST`"]
pub struct I2S_IN_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_IN_LOOP_TEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `I2S_OUT_LOOP_TEST`"]
pub type I2S_OUT_LOOP_TEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_OUT_LOOP_TEST`"]
pub struct I2S_OUT_LOOP_TEST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_OUT_LOOP_TEST_W<'a> {
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
#[doc = "Reader of field `I2S_AHBM_RST`"]
pub type I2S_AHBM_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_AHBM_RST`"]
pub struct I2S_AHBM_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_AHBM_RST_W<'a> {
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
#[doc = "Reader of field `I2S_AHBM_FIFO_RST`"]
pub type I2S_AHBM_FIFO_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_AHBM_FIFO_RST`"]
pub struct I2S_AHBM_FIFO_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_AHBM_FIFO_RST_W<'a> {
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
#[doc = "Reader of field `I2S_OUT_RST`"]
pub type I2S_OUT_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_OUT_RST`"]
pub struct I2S_OUT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_OUT_RST_W<'a> {
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
#[doc = "Reader of field `I2S_IN_RST`"]
pub type I2S_IN_RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_IN_RST`"]
pub struct I2S_IN_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_IN_RST_W<'a> {
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
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2s_mem_trans_en(&self) -> I2S_MEM_TRANS_EN_R {
        I2S_MEM_TRANS_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s_check_owner(&self) -> I2S_CHECK_OWNER_R {
        I2S_CHECK_OWNER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s_out_data_burst_en(&self) -> I2S_OUT_DATA_BURST_EN_R {
        I2S_OUT_DATA_BURST_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s_indscr_burst_en(&self) -> I2S_INDSCR_BURST_EN_R {
        I2S_INDSCR_BURST_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2s_outdscr_burst_en(&self) -> I2S_OUTDSCR_BURST_EN_R {
        I2S_OUTDSCR_BURST_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2s_out_eof_mode(&self) -> I2S_OUT_EOF_MODE_R {
        I2S_OUT_EOF_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s_out_no_restart_clr(&self) -> I2S_OUT_NO_RESTART_CLR_R {
        I2S_OUT_NO_RESTART_CLR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s_out_auto_wrback(&self) -> I2S_OUT_AUTO_WRBACK_R {
        I2S_OUT_AUTO_WRBACK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s_in_loop_test(&self) -> I2S_IN_LOOP_TEST_R {
        I2S_IN_LOOP_TEST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s_out_loop_test(&self) -> I2S_OUT_LOOP_TEST_R {
        I2S_OUT_LOOP_TEST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_ahbm_rst(&self) -> I2S_AHBM_RST_R {
        I2S_AHBM_RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_ahbm_fifo_rst(&self) -> I2S_AHBM_FIFO_RST_R {
        I2S_AHBM_FIFO_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_out_rst(&self) -> I2S_OUT_RST_R {
        I2S_OUT_RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_in_rst(&self) -> I2S_IN_RST_R {
        I2S_IN_RST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn i2s_mem_trans_en(&mut self) -> I2S_MEM_TRANS_EN_W {
        I2S_MEM_TRANS_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn i2s_check_owner(&mut self) -> I2S_CHECK_OWNER_W {
        I2S_CHECK_OWNER_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn i2s_out_data_burst_en(&mut self) -> I2S_OUT_DATA_BURST_EN_W {
        I2S_OUT_DATA_BURST_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn i2s_indscr_burst_en(&mut self) -> I2S_INDSCR_BURST_EN_W {
        I2S_INDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn i2s_outdscr_burst_en(&mut self) -> I2S_OUTDSCR_BURST_EN_W {
        I2S_OUTDSCR_BURST_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn i2s_out_eof_mode(&mut self) -> I2S_OUT_EOF_MODE_W {
        I2S_OUT_EOF_MODE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn i2s_out_no_restart_clr(&mut self) -> I2S_OUT_NO_RESTART_CLR_W {
        I2S_OUT_NO_RESTART_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn i2s_out_auto_wrback(&mut self) -> I2S_OUT_AUTO_WRBACK_W {
        I2S_OUT_AUTO_WRBACK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2s_in_loop_test(&mut self) -> I2S_IN_LOOP_TEST_W {
        I2S_IN_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2s_out_loop_test(&mut self) -> I2S_OUT_LOOP_TEST_W {
        I2S_OUT_LOOP_TEST_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2s_ahbm_rst(&mut self) -> I2S_AHBM_RST_W {
        I2S_AHBM_RST_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2s_ahbm_fifo_rst(&mut self) -> I2S_AHBM_FIFO_RST_W {
        I2S_AHBM_FIFO_RST_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2s_out_rst(&mut self) -> I2S_OUT_RST_W {
        I2S_OUT_RST_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2s_in_rst(&mut self) -> I2S_IN_RST_W {
        I2S_IN_RST_W { w: self }
    }
}
