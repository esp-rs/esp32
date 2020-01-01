#[doc = "Reader of register CTRL2"]
pub type R = crate::R<u32, super::CTRL2>;
#[doc = "Writer for register CTRL2"]
pub type W = crate::W<u32, super::CTRL2>;
#[doc = "Register CTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_CS_DELAY_NUM`"]
pub type SPI_CS_DELAY_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CS_DELAY_NUM`"]
pub struct SPI_CS_DELAY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_DELAY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `SPI_CS_DELAY_MODE`"]
pub type SPI_CS_DELAY_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CS_DELAY_MODE`"]
pub struct SPI_CS_DELAY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CS_DELAY_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `SPI_MOSI_DELAY_NUM`"]
pub type SPI_MOSI_DELAY_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MOSI_DELAY_NUM`"]
pub struct SPI_MOSI_DELAY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MOSI_DELAY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | (((value as u32) & 0x07) << 23);
        self.w
    }
}
#[doc = "Reader of field `SPI_MOSI_DELAY_MODE`"]
pub type SPI_MOSI_DELAY_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MOSI_DELAY_MODE`"]
pub struct SPI_MOSI_DELAY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MOSI_DELAY_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `SPI_MISO_DELAY_NUM`"]
pub type SPI_MISO_DELAY_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MISO_DELAY_NUM`"]
pub struct SPI_MISO_DELAY_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MISO_DELAY_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `SPI_MISO_DELAY_MODE`"]
pub type SPI_MISO_DELAY_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_MISO_DELAY_MODE`"]
pub struct SPI_MISO_DELAY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MISO_DELAY_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI_CK_OUT_HIGH_MODE`"]
pub type SPI_CK_OUT_HIGH_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CK_OUT_HIGH_MODE`"]
pub struct SPI_CK_OUT_HIGH_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CK_OUT_HIGH_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `SPI_CK_OUT_LOW_MODE`"]
pub type SPI_CK_OUT_LOW_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_CK_OUT_LOW_MODE`"]
pub struct SPI_CK_OUT_LOW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CK_OUT_LOW_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI_HOLD_TIME`"]
pub type SPI_HOLD_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_HOLD_TIME`"]
pub struct SPI_HOLD_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_HOLD_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPI_SETUP_TIME`"]
pub type SPI_SETUP_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI_SETUP_TIME`"]
pub struct SPI_SETUP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SETUP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - spi_cs signal is delayed by system clock cycles"]
    #[inline(always)]
    pub fn spi_cs_delay_num(&self) -> SPI_CS_DELAY_NUM_R {
        SPI_CS_DELAY_NUM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 26:27 - spi_cs signal is delayed by spi_clk . 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn spi_cs_delay_mode(&self) -> SPI_CS_DELAY_MODE_R {
        SPI_CS_DELAY_MODE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 23:25 - MOSI signals are delayed by system clock cycles"]
    #[inline(always)]
    pub fn spi_mosi_delay_num(&self) -> SPI_MOSI_DELAY_NUM_R {
        SPI_MOSI_DELAY_NUM_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 21:22 - MOSI signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn spi_mosi_delay_mode(&self) -> SPI_MOSI_DELAY_MODE_R {
        SPI_MOSI_DELAY_MODE_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - MISO signals are delayed by system clock cycles"]
    #[inline(always)]
    pub fn spi_miso_delay_num(&self) -> SPI_MISO_DELAY_NUM_R {
        SPI_MISO_DELAY_NUM_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - MISO signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn spi_miso_delay_mode(&self) -> SPI_MISO_DELAY_MODE_R {
        SPI_MISO_DELAY_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_H bits."]
    #[inline(always)]
    pub fn spi_ck_out_high_mode(&self) -> SPI_CK_OUT_HIGH_MODE_R {
        SPI_CK_OUT_HIGH_MODE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_L bits."]
    #[inline(always)]
    pub fn spi_ck_out_low_mode(&self) -> SPI_CK_OUT_LOW_MODE_R {
        SPI_CK_OUT_LOW_MODE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - delay cycles of cs pin by spi clock, this bits combined with spi_cs_hold bit."]
    #[inline(always)]
    pub fn spi_hold_time(&self) -> SPI_HOLD_TIME_R {
        SPI_HOLD_TIME_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - (cycles-1) of \u{a1}\u{b0}prepare\u{a1}\u{b1} phase by spi clock, this bits combined with spi_cs_setup bit."]
    #[inline(always)]
    pub fn spi_setup_time(&self) -> SPI_SETUP_TIME_R {
        SPI_SETUP_TIME_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - spi_cs signal is delayed by system clock cycles"]
    #[inline(always)]
    pub fn spi_cs_delay_num(&mut self) -> SPI_CS_DELAY_NUM_W {
        SPI_CS_DELAY_NUM_W { w: self }
    }
    #[doc = "Bits 26:27 - spi_cs signal is delayed by spi_clk . 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn spi_cs_delay_mode(&mut self) -> SPI_CS_DELAY_MODE_W {
        SPI_CS_DELAY_MODE_W { w: self }
    }
    #[doc = "Bits 23:25 - MOSI signals are delayed by system clock cycles"]
    #[inline(always)]
    pub fn spi_mosi_delay_num(&mut self) -> SPI_MOSI_DELAY_NUM_W {
        SPI_MOSI_DELAY_NUM_W { w: self }
    }
    #[doc = "Bits 21:22 - MOSI signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn spi_mosi_delay_mode(&mut self) -> SPI_MOSI_DELAY_MODE_W {
        SPI_MOSI_DELAY_MODE_W { w: self }
    }
    #[doc = "Bits 18:20 - MISO signals are delayed by system clock cycles"]
    #[inline(always)]
    pub fn spi_miso_delay_num(&mut self) -> SPI_MISO_DELAY_NUM_W {
        SPI_MISO_DELAY_NUM_W { w: self }
    }
    #[doc = "Bits 16:17 - MISO signals are delayed by spi_clk. 0: zero 1: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by half cycle else delayed by one cycle 2: if spi_ck_out_edge or spi_ck_i_edge is set 1 delayed by one cycle else delayed by half cycle 3: delayed one cycle"]
    #[inline(always)]
    pub fn spi_miso_delay_mode(&mut self) -> SPI_MISO_DELAY_MODE_W {
        SPI_MISO_DELAY_MODE_W { w: self }
    }
    #[doc = "Bits 12:15 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_H bits."]
    #[inline(always)]
    pub fn spi_ck_out_high_mode(&mut self) -> SPI_CK_OUT_HIGH_MODE_W {
        SPI_CK_OUT_HIGH_MODE_W { w: self }
    }
    #[doc = "Bits 8:11 - modify spi clock duty ratio when the value is lager than 8, the bits are combined with spi_clkcnt_N bits and spi_clkcnt_L bits."]
    #[inline(always)]
    pub fn spi_ck_out_low_mode(&mut self) -> SPI_CK_OUT_LOW_MODE_W {
        SPI_CK_OUT_LOW_MODE_W { w: self }
    }
    #[doc = "Bits 4:7 - delay cycles of cs pin by spi clock, this bits combined with spi_cs_hold bit."]
    #[inline(always)]
    pub fn spi_hold_time(&mut self) -> SPI_HOLD_TIME_W {
        SPI_HOLD_TIME_W { w: self }
    }
    #[doc = "Bits 0:3 - (cycles-1) of \u{a1}\u{b0}prepare\u{a1}\u{b1} phase by spi clock, this bits combined with spi_cs_setup bit."]
    #[inline(always)]
    pub fn spi_setup_time(&mut self) -> SPI_SETUP_TIME_W {
        SPI_SETUP_TIME_W { w: self }
    }
}
