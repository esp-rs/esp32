#[doc = "Reader of register CVSD_CONF2"]
pub type R = crate::R<u32, super::CVSD_CONF2>;
#[doc = "Writer for register CVSD_CONF2"]
pub type W = crate::W<u32, super::CVSD_CONF2>;
#[doc = "Register CVSD_CONF2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CVSD_CONF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CVSD_H`"]
pub type CVSD_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CVSD_H`"]
pub struct CVSD_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSD_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `CVSD_BETA`"]
pub type CVSD_BETA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CVSD_BETA`"]
pub struct CVSD_BETA_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSD_BETA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 6)) | (((value as u32) & 0x03ff) << 6);
        self.w
    }
}
#[doc = "Reader of field `CVSD_J`"]
pub type CVSD_J_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CVSD_J`"]
pub struct CVSD_J_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSD_J_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `CVSD_K`"]
pub type CVSD_K_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CVSD_K`"]
pub struct CVSD_K_W<'a> {
    w: &'a mut W,
}
impl<'a> CVSD_K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cvsd_h(&self) -> CVSD_H_R {
        CVSD_H_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn cvsd_beta(&self) -> CVSD_BETA_R {
        CVSD_BETA_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn cvsd_j(&self) -> CVSD_J_R {
        CVSD_J_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cvsd_k(&self) -> CVSD_K_R {
        CVSD_K_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cvsd_h(&mut self) -> CVSD_H_W {
        CVSD_H_W { w: self }
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn cvsd_beta(&mut self) -> CVSD_BETA_W {
        CVSD_BETA_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn cvsd_j(&mut self) -> CVSD_J_W {
        CVSD_J_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cvsd_k(&mut self) -> CVSD_K_W {
        CVSD_K_W { w: self }
    }
}
