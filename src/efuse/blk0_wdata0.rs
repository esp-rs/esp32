#[doc = "Reader of register BLK0_WDATA0"]
pub type R = crate::R<u32, super::BLK0_WDATA0>;
#[doc = "Writer for register BLK0_WDATA0"]
pub type W = crate::W<u32, super::BLK0_WDATA0>;
#[doc = "Register BLK0_WDATA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::BLK0_WDATA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLASH_CRYPT_CNT`"]
pub type FLASH_CRYPT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLASH_CRYPT_CNT`"]
pub struct FLASH_CRYPT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_CRYPT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 20)) | (((value as u32) & 0x7f) << 20);
        self.w
    }
}
#[doc = "Reader of field `RD_DIS`"]
pub type RD_DIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RD_DIS`"]
pub struct RD_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `WR_DIS`"]
pub type WR_DIS_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WR_DIS`"]
pub struct WR_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:26 - program for flash_crypt_cnt"]
    #[inline(always)]
    pub fn flash_crypt_cnt(&self) -> FLASH_CRYPT_CNT_R {
        FLASH_CRYPT_CNT_R::new(((self.bits >> 20) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - program for efuse_rd_disable"]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 0:15 - program for efuse_wr_disable"]
    #[inline(always)]
    pub fn wr_dis(&self) -> WR_DIS_R {
        WR_DIS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:26 - program for flash_crypt_cnt"]
    #[inline(always)]
    pub fn flash_crypt_cnt(&mut self) -> FLASH_CRYPT_CNT_W {
        FLASH_CRYPT_CNT_W { w: self }
    }
    #[doc = "Bits 16:19 - program for efuse_rd_disable"]
    #[inline(always)]
    pub fn rd_dis(&mut self) -> RD_DIS_W {
        RD_DIS_W { w: self }
    }
    #[doc = "Bits 0:15 - program for efuse_wr_disable"]
    #[inline(always)]
    pub fn wr_dis(&mut self) -> WR_DIS_W {
        WR_DIS_W { w: self }
    }
}
