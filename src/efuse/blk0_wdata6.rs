#[doc = "Reader of register BLK0_WDATA6"]
pub type R = crate::R<u32, super::BLK0_WDATA6>;
#[doc = "Writer for register BLK0_WDATA6"]
pub type W = crate::W<u32, super::BLK0_WDATA6>;
#[doc = "Register BLK0_WDATA6 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK0_WDATA6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEY_STATUS`"]
pub type KEY_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEY_STATUS`"]
pub struct KEY_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_STATUS_W<'a> {
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
#[doc = "Reader of field `DISABLE_DL_CACHE`"]
pub type DISABLE_DL_CACHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_DL_CACHE`"]
pub struct DISABLE_DL_CACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_DL_CACHE_W<'a> {
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
#[doc = "Reader of field `DISABLE_DL_DECRYPT`"]
pub type DISABLE_DL_DECRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_DL_DECRYPT`"]
pub struct DISABLE_DL_DECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_DL_DECRYPT_W<'a> {
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
#[doc = "Reader of field `DISABLE_DL_ENCRYPT`"]
pub type DISABLE_DL_ENCRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_DL_ENCRYPT`"]
pub struct DISABLE_DL_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_DL_ENCRYPT_W<'a> {
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
#[doc = "Reader of field `DISABLE_JTAG`"]
pub type DISABLE_JTAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_JTAG`"]
pub struct DISABLE_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_JTAG_W<'a> {
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
#[doc = "Reader of field `ABS_DONE_1`"]
pub type ABS_DONE_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABS_DONE_1`"]
pub struct ABS_DONE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ABS_DONE_1_W<'a> {
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
#[doc = "Reader of field `ABS_DONE_0`"]
pub type ABS_DONE_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABS_DONE_0`"]
pub struct ABS_DONE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ABS_DONE_0_W<'a> {
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
#[doc = "Reader of field `DISABLE_SDIO_HOST`"]
pub type DISABLE_SDIO_HOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLE_SDIO_HOST`"]
pub struct DISABLE_SDIO_HOST_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_SDIO_HOST_W<'a> {
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
#[doc = "Reader of field `CONSOLE_DEBUG_DISABLE`"]
pub type CONSOLE_DEBUG_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONSOLE_DEBUG_DISABLE`"]
pub struct CONSOLE_DEBUG_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSOLE_DEBUG_DISABLE_W<'a> {
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
#[doc = "Reader of field `CODING_SCHEME`"]
pub type CODING_SCHEME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CODING_SCHEME`"]
pub struct CODING_SCHEME_W<'a> {
    w: &'a mut W,
}
impl<'a> CODING_SCHEME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 10 - program for key_status"]
    #[inline(always)]
    pub fn key_status(&self) -> KEY_STATUS_R {
        KEY_STATUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - program for download_dis_cache"]
    #[inline(always)]
    pub fn disable_dl_cache(&self) -> DISABLE_DL_CACHE_R {
        DISABLE_DL_CACHE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - program for download_dis_decrypt"]
    #[inline(always)]
    pub fn disable_dl_decrypt(&self) -> DISABLE_DL_DECRYPT_R {
        DISABLE_DL_DECRYPT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - program for download_dis_encrypt"]
    #[inline(always)]
    pub fn disable_dl_encrypt(&self) -> DISABLE_DL_ENCRYPT_R {
        DISABLE_DL_ENCRYPT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - program for JTAG_disable"]
    #[inline(always)]
    pub fn disable_jtag(&self) -> DISABLE_JTAG_R {
        DISABLE_JTAG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - program for abstract_done_1"]
    #[inline(always)]
    pub fn abs_done_1(&self) -> ABS_DONE_1_R {
        ABS_DONE_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - program for abstract_done_0"]
    #[inline(always)]
    pub fn abs_done_0(&self) -> ABS_DONE_0_R {
        ABS_DONE_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn disable_sdio_host(&self) -> DISABLE_SDIO_HOST_R {
        DISABLE_SDIO_HOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - program for console_debug_disable"]
    #[inline(always)]
    pub fn console_debug_disable(&self) -> CONSOLE_DEBUG_DISABLE_R {
        CONSOLE_DEBUG_DISABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - program for coding_scheme"]
    #[inline(always)]
    pub fn coding_scheme(&self) -> CODING_SCHEME_R {
        CODING_SCHEME_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 10 - program for key_status"]
    #[inline(always)]
    pub fn key_status(&mut self) -> KEY_STATUS_W {
        KEY_STATUS_W { w: self }
    }
    #[doc = "Bit 9 - program for download_dis_cache"]
    #[inline(always)]
    pub fn disable_dl_cache(&mut self) -> DISABLE_DL_CACHE_W {
        DISABLE_DL_CACHE_W { w: self }
    }
    #[doc = "Bit 8 - program for download_dis_decrypt"]
    #[inline(always)]
    pub fn disable_dl_decrypt(&mut self) -> DISABLE_DL_DECRYPT_W {
        DISABLE_DL_DECRYPT_W { w: self }
    }
    #[doc = "Bit 7 - program for download_dis_encrypt"]
    #[inline(always)]
    pub fn disable_dl_encrypt(&mut self) -> DISABLE_DL_ENCRYPT_W {
        DISABLE_DL_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 6 - program for JTAG_disable"]
    #[inline(always)]
    pub fn disable_jtag(&mut self) -> DISABLE_JTAG_W {
        DISABLE_JTAG_W { w: self }
    }
    #[doc = "Bit 5 - program for abstract_done_1"]
    #[inline(always)]
    pub fn abs_done_1(&mut self) -> ABS_DONE_1_W {
        ABS_DONE_1_W { w: self }
    }
    #[doc = "Bit 4 - program for abstract_done_0"]
    #[inline(always)]
    pub fn abs_done_0(&mut self) -> ABS_DONE_0_W {
        ABS_DONE_0_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn disable_sdio_host(&mut self) -> DISABLE_SDIO_HOST_W {
        DISABLE_SDIO_HOST_W { w: self }
    }
    #[doc = "Bit 2 - program for console_debug_disable"]
    #[inline(always)]
    pub fn console_debug_disable(&mut self) -> CONSOLE_DEBUG_DISABLE_W {
        CONSOLE_DEBUG_DISABLE_W { w: self }
    }
    #[doc = "Bits 0:1 - program for coding_scheme"]
    #[inline(always)]
    pub fn coding_scheme(&mut self) -> CODING_SCHEME_W {
        CODING_SCHEME_W { w: self }
    }
}
