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
#[doc = "Reader of field `EFUSE_KEY_STATUS`"]
pub type EFUSE_KEY_STATUS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_KEY_STATUS`"]
pub struct EFUSE_KEY_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_KEY_STATUS_W<'a> {
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
#[doc = "Reader of field `EFUSE_DISABLE_DL_CACHE`"]
pub type EFUSE_DISABLE_DL_CACHE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DISABLE_DL_CACHE`"]
pub struct EFUSE_DISABLE_DL_CACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DISABLE_DL_CACHE_W<'a> {
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
#[doc = "Reader of field `EFUSE_DISABLE_DL_DECRYPT`"]
pub type EFUSE_DISABLE_DL_DECRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DISABLE_DL_DECRYPT`"]
pub struct EFUSE_DISABLE_DL_DECRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DISABLE_DL_DECRYPT_W<'a> {
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
#[doc = "Reader of field `EFUSE_DISABLE_DL_ENCRYPT`"]
pub type EFUSE_DISABLE_DL_ENCRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DISABLE_DL_ENCRYPT`"]
pub struct EFUSE_DISABLE_DL_ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DISABLE_DL_ENCRYPT_W<'a> {
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
#[doc = "Reader of field `EFUSE_DISABLE_JTAG`"]
pub type EFUSE_DISABLE_JTAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DISABLE_JTAG`"]
pub struct EFUSE_DISABLE_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DISABLE_JTAG_W<'a> {
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
#[doc = "Reader of field `EFUSE_ABS_DONE_1`"]
pub type EFUSE_ABS_DONE_1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_ABS_DONE_1`"]
pub struct EFUSE_ABS_DONE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_ABS_DONE_1_W<'a> {
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
#[doc = "Reader of field `EFUSE_ABS_DONE_0`"]
pub type EFUSE_ABS_DONE_0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_ABS_DONE_0`"]
pub struct EFUSE_ABS_DONE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_ABS_DONE_0_W<'a> {
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
#[doc = "Reader of field `EFUSE_DISABLE_SDIO_HOST`"]
pub type EFUSE_DISABLE_SDIO_HOST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_DISABLE_SDIO_HOST`"]
pub struct EFUSE_DISABLE_SDIO_HOST_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_DISABLE_SDIO_HOST_W<'a> {
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
#[doc = "Reader of field `EFUSE_CONSOLE_DEBUG_DISABLE`"]
pub type EFUSE_CONSOLE_DEBUG_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EFUSE_CONSOLE_DEBUG_DISABLE`"]
pub struct EFUSE_CONSOLE_DEBUG_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_CONSOLE_DEBUG_DISABLE_W<'a> {
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
#[doc = "Reader of field `EFUSE_CODING_SCHEME`"]
pub type EFUSE_CODING_SCHEME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_CODING_SCHEME`"]
pub struct EFUSE_CODING_SCHEME_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_CODING_SCHEME_W<'a> {
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
    pub fn efuse_key_status(&self) -> EFUSE_KEY_STATUS_R {
        EFUSE_KEY_STATUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - program for download_dis_cache"]
    #[inline(always)]
    pub fn efuse_disable_dl_cache(&self) -> EFUSE_DISABLE_DL_CACHE_R {
        EFUSE_DISABLE_DL_CACHE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - program for download_dis_decrypt"]
    #[inline(always)]
    pub fn efuse_disable_dl_decrypt(&self) -> EFUSE_DISABLE_DL_DECRYPT_R {
        EFUSE_DISABLE_DL_DECRYPT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - program for download_dis_encrypt"]
    #[inline(always)]
    pub fn efuse_disable_dl_encrypt(&self) -> EFUSE_DISABLE_DL_ENCRYPT_R {
        EFUSE_DISABLE_DL_ENCRYPT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - program for JTAG_disable"]
    #[inline(always)]
    pub fn efuse_disable_jtag(&self) -> EFUSE_DISABLE_JTAG_R {
        EFUSE_DISABLE_JTAG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - program for abstract_done_1"]
    #[inline(always)]
    pub fn efuse_abs_done_1(&self) -> EFUSE_ABS_DONE_1_R {
        EFUSE_ABS_DONE_1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - program for abstract_done_0"]
    #[inline(always)]
    pub fn efuse_abs_done_0(&self) -> EFUSE_ABS_DONE_0_R {
        EFUSE_ABS_DONE_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_disable_sdio_host(&self) -> EFUSE_DISABLE_SDIO_HOST_R {
        EFUSE_DISABLE_SDIO_HOST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - program for console_debug_disable"]
    #[inline(always)]
    pub fn efuse_console_debug_disable(&self) -> EFUSE_CONSOLE_DEBUG_DISABLE_R {
        EFUSE_CONSOLE_DEBUG_DISABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 0:1 - program for coding_scheme"]
    #[inline(always)]
    pub fn efuse_coding_scheme(&self) -> EFUSE_CODING_SCHEME_R {
        EFUSE_CODING_SCHEME_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 10 - program for key_status"]
    #[inline(always)]
    pub fn efuse_key_status(&mut self) -> EFUSE_KEY_STATUS_W {
        EFUSE_KEY_STATUS_W { w: self }
    }
    #[doc = "Bit 9 - program for download_dis_cache"]
    #[inline(always)]
    pub fn efuse_disable_dl_cache(&mut self) -> EFUSE_DISABLE_DL_CACHE_W {
        EFUSE_DISABLE_DL_CACHE_W { w: self }
    }
    #[doc = "Bit 8 - program for download_dis_decrypt"]
    #[inline(always)]
    pub fn efuse_disable_dl_decrypt(&mut self) -> EFUSE_DISABLE_DL_DECRYPT_W {
        EFUSE_DISABLE_DL_DECRYPT_W { w: self }
    }
    #[doc = "Bit 7 - program for download_dis_encrypt"]
    #[inline(always)]
    pub fn efuse_disable_dl_encrypt(&mut self) -> EFUSE_DISABLE_DL_ENCRYPT_W {
        EFUSE_DISABLE_DL_ENCRYPT_W { w: self }
    }
    #[doc = "Bit 6 - program for JTAG_disable"]
    #[inline(always)]
    pub fn efuse_disable_jtag(&mut self) -> EFUSE_DISABLE_JTAG_W {
        EFUSE_DISABLE_JTAG_W { w: self }
    }
    #[doc = "Bit 5 - program for abstract_done_1"]
    #[inline(always)]
    pub fn efuse_abs_done_1(&mut self) -> EFUSE_ABS_DONE_1_W {
        EFUSE_ABS_DONE_1_W { w: self }
    }
    #[doc = "Bit 4 - program for abstract_done_0"]
    #[inline(always)]
    pub fn efuse_abs_done_0(&mut self) -> EFUSE_ABS_DONE_0_W {
        EFUSE_ABS_DONE_0_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_disable_sdio_host(&mut self) -> EFUSE_DISABLE_SDIO_HOST_W {
        EFUSE_DISABLE_SDIO_HOST_W { w: self }
    }
    #[doc = "Bit 2 - program for console_debug_disable"]
    #[inline(always)]
    pub fn efuse_console_debug_disable(&mut self) -> EFUSE_CONSOLE_DEBUG_DISABLE_W {
        EFUSE_CONSOLE_DEBUG_DISABLE_W { w: self }
    }
    #[doc = "Bits 0:1 - program for coding_scheme"]
    #[inline(always)]
    pub fn efuse_coding_scheme(&mut self) -> EFUSE_CODING_SCHEME_W {
        EFUSE_CODING_SCHEME_W { w: self }
    }
}
