#[doc = "Reader of register PLC_CONF1"]
pub type R = crate::R<u32, super::PLC_CONF1>;
#[doc = "Writer for register PLC_CONF1"]
pub type W = crate::W<u32, super::PLC_CONF1>;
#[doc = "Register PLC_CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PLC_CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLIDE_WIN_LEN`"]
pub type SLIDE_WIN_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLIDE_WIN_LEN`"]
pub struct SLIDE_WIN_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLIDE_WIN_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `BAD_OLA_WIN2_PARA`"]
pub type BAD_OLA_WIN2_PARA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BAD_OLA_WIN2_PARA`"]
pub struct BAD_OLA_WIN2_PARA_W<'a> {
    w: &'a mut W,
}
impl<'a> BAD_OLA_WIN2_PARA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `BAD_OLA_WIN2_PARA_SHIFT`"]
pub type BAD_OLA_WIN2_PARA_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BAD_OLA_WIN2_PARA_SHIFT`"]
pub struct BAD_OLA_WIN2_PARA_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> BAD_OLA_WIN2_PARA_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `BAD_CEF_ATTEN_PARA_SHIFT`"]
pub type BAD_CEF_ATTEN_PARA_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BAD_CEF_ATTEN_PARA_SHIFT`"]
pub struct BAD_CEF_ATTEN_PARA_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> BAD_CEF_ATTEN_PARA_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `BAD_CEF_ATTEN_PARA`"]
pub type BAD_CEF_ATTEN_PARA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BAD_CEF_ATTEN_PARA`"]
pub struct BAD_CEF_ATTEN_PARA_W<'a> {
    w: &'a mut W,
}
impl<'a> BAD_CEF_ATTEN_PARA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn slide_win_len(&self) -> SLIDE_WIN_LEN_R {
        SLIDE_WIN_LEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn bad_ola_win2_para(&self) -> BAD_OLA_WIN2_PARA_R {
        BAD_OLA_WIN2_PARA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn bad_ola_win2_para_shift(&self) -> BAD_OLA_WIN2_PARA_SHIFT_R {
        BAD_OLA_WIN2_PARA_SHIFT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn bad_cef_atten_para_shift(&self) -> BAD_CEF_ATTEN_PARA_SHIFT_R {
        BAD_CEF_ATTEN_PARA_SHIFT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bad_cef_atten_para(&self) -> BAD_CEF_ATTEN_PARA_R {
        BAD_CEF_ATTEN_PARA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn slide_win_len(&mut self) -> SLIDE_WIN_LEN_W {
        SLIDE_WIN_LEN_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn bad_ola_win2_para(&mut self) -> BAD_OLA_WIN2_PARA_W {
        BAD_OLA_WIN2_PARA_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn bad_ola_win2_para_shift(&mut self) -> BAD_OLA_WIN2_PARA_SHIFT_W {
        BAD_OLA_WIN2_PARA_SHIFT_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn bad_cef_atten_para_shift(&mut self) -> BAD_CEF_ATTEN_PARA_SHIFT_W {
        BAD_CEF_ATTEN_PARA_SHIFT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn bad_cef_atten_para(&mut self) -> BAD_CEF_ATTEN_PARA_W {
        BAD_CEF_ATTEN_PARA_W { w: self }
    }
}
