#[doc = "Register `YSELECT` reader"]
pub type R = crate::R<YselectSpec>;
#[doc = "Register `YSELECT` writer"]
pub type W = crate::W<YselectSpec>;
#[doc = "Y line selection MUX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Ymuxselect {
    #[doc = "1: PTC Y0 Pin"]
    Y0 = 1,
    #[doc = "2: PTC Y1 Pin"]
    Y1 = 2,
    #[doc = "4: PTC Y2 Pin"]
    Y2 = 4,
    #[doc = "8: PTC Y3 Pin"]
    Y3 = 8,
    #[doc = "16: PTC Y4 Pin"]
    Y4 = 16,
    #[doc = "32: PTC Y5 Pin"]
    Y5 = 32,
    #[doc = "64: PTC Y6 Pin"]
    Y6 = 64,
    #[doc = "128: PTC Y7 Pin"]
    Y7 = 128,
    #[doc = "256: PTC Y8 Pin"]
    Y8 = 256,
    #[doc = "512: PTC Y9 Pin"]
    Y9 = 512,
    #[doc = "1024: PTC Y10 Pin"]
    Y10 = 1024,
    #[doc = "2048: PTC Y11 Pin"]
    Y11 = 2048,
    #[doc = "4096: PTC Y12 Pin"]
    Y12 = 4096,
    #[doc = "8192: PTC Y13 Pin"]
    Y13 = 8192,
    #[doc = "16384: PTC Y14 Pin"]
    Y14 = 16384,
    #[doc = "32768: PTC Y15 Pin"]
    Y15 = 32768,
}
impl From<Ymuxselect> for u16 {
    #[inline(always)]
    fn from(variant: Ymuxselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ymuxselect {
    type Ux = u16;
}
impl crate::IsEnum for Ymuxselect {}
#[doc = "Field `YMUX` reader - Y line selection MUX"]
pub type YmuxR = crate::FieldReader<Ymuxselect>;
impl YmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ymuxselect> {
        match self.bits {
            1 => Some(Ymuxselect::Y0),
            2 => Some(Ymuxselect::Y1),
            4 => Some(Ymuxselect::Y2),
            8 => Some(Ymuxselect::Y3),
            16 => Some(Ymuxselect::Y4),
            32 => Some(Ymuxselect::Y5),
            64 => Some(Ymuxselect::Y6),
            128 => Some(Ymuxselect::Y7),
            256 => Some(Ymuxselect::Y8),
            512 => Some(Ymuxselect::Y9),
            1024 => Some(Ymuxselect::Y10),
            2048 => Some(Ymuxselect::Y11),
            4096 => Some(Ymuxselect::Y12),
            8192 => Some(Ymuxselect::Y13),
            16384 => Some(Ymuxselect::Y14),
            32768 => Some(Ymuxselect::Y15),
            _ => None,
        }
    }
    #[doc = "PTC Y0 Pin"]
    #[inline(always)]
    pub fn is_y0(&self) -> bool {
        *self == Ymuxselect::Y0
    }
    #[doc = "PTC Y1 Pin"]
    #[inline(always)]
    pub fn is_y1(&self) -> bool {
        *self == Ymuxselect::Y1
    }
    #[doc = "PTC Y2 Pin"]
    #[inline(always)]
    pub fn is_y2(&self) -> bool {
        *self == Ymuxselect::Y2
    }
    #[doc = "PTC Y3 Pin"]
    #[inline(always)]
    pub fn is_y3(&self) -> bool {
        *self == Ymuxselect::Y3
    }
    #[doc = "PTC Y4 Pin"]
    #[inline(always)]
    pub fn is_y4(&self) -> bool {
        *self == Ymuxselect::Y4
    }
    #[doc = "PTC Y5 Pin"]
    #[inline(always)]
    pub fn is_y5(&self) -> bool {
        *self == Ymuxselect::Y5
    }
    #[doc = "PTC Y6 Pin"]
    #[inline(always)]
    pub fn is_y6(&self) -> bool {
        *self == Ymuxselect::Y6
    }
    #[doc = "PTC Y7 Pin"]
    #[inline(always)]
    pub fn is_y7(&self) -> bool {
        *self == Ymuxselect::Y7
    }
    #[doc = "PTC Y8 Pin"]
    #[inline(always)]
    pub fn is_y8(&self) -> bool {
        *self == Ymuxselect::Y8
    }
    #[doc = "PTC Y9 Pin"]
    #[inline(always)]
    pub fn is_y9(&self) -> bool {
        *self == Ymuxselect::Y9
    }
    #[doc = "PTC Y10 Pin"]
    #[inline(always)]
    pub fn is_y10(&self) -> bool {
        *self == Ymuxselect::Y10
    }
    #[doc = "PTC Y11 Pin"]
    #[inline(always)]
    pub fn is_y11(&self) -> bool {
        *self == Ymuxselect::Y11
    }
    #[doc = "PTC Y12 Pin"]
    #[inline(always)]
    pub fn is_y12(&self) -> bool {
        *self == Ymuxselect::Y12
    }
    #[doc = "PTC Y13 Pin"]
    #[inline(always)]
    pub fn is_y13(&self) -> bool {
        *self == Ymuxselect::Y13
    }
    #[doc = "PTC Y14 Pin"]
    #[inline(always)]
    pub fn is_y14(&self) -> bool {
        *self == Ymuxselect::Y14
    }
    #[doc = "PTC Y15 Pin"]
    #[inline(always)]
    pub fn is_y15(&self) -> bool {
        *self == Ymuxselect::Y15
    }
}
#[doc = "Field `YMUX` writer - Y line selection MUX"]
pub type YmuxW<'a, REG> = crate::FieldWriter<'a, REG, 16, Ymuxselect>;
impl<'a, REG> YmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "PTC Y0 Pin"]
    #[inline(always)]
    pub fn y0(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y0)
    }
    #[doc = "PTC Y1 Pin"]
    #[inline(always)]
    pub fn y1(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y1)
    }
    #[doc = "PTC Y2 Pin"]
    #[inline(always)]
    pub fn y2(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y2)
    }
    #[doc = "PTC Y3 Pin"]
    #[inline(always)]
    pub fn y3(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y3)
    }
    #[doc = "PTC Y4 Pin"]
    #[inline(always)]
    pub fn y4(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y4)
    }
    #[doc = "PTC Y5 Pin"]
    #[inline(always)]
    pub fn y5(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y5)
    }
    #[doc = "PTC Y6 Pin"]
    #[inline(always)]
    pub fn y6(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y6)
    }
    #[doc = "PTC Y7 Pin"]
    #[inline(always)]
    pub fn y7(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y7)
    }
    #[doc = "PTC Y8 Pin"]
    #[inline(always)]
    pub fn y8(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y8)
    }
    #[doc = "PTC Y9 Pin"]
    #[inline(always)]
    pub fn y9(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y9)
    }
    #[doc = "PTC Y10 Pin"]
    #[inline(always)]
    pub fn y10(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y10)
    }
    #[doc = "PTC Y11 Pin"]
    #[inline(always)]
    pub fn y11(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y11)
    }
    #[doc = "PTC Y12 Pin"]
    #[inline(always)]
    pub fn y12(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y12)
    }
    #[doc = "PTC Y13 Pin"]
    #[inline(always)]
    pub fn y13(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y13)
    }
    #[doc = "PTC Y14 Pin"]
    #[inline(always)]
    pub fn y14(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y14)
    }
    #[doc = "PTC Y15 Pin"]
    #[inline(always)]
    pub fn y15(self) -> &'a mut crate::W<REG> {
        self.variant(Ymuxselect::Y15)
    }
}
impl R {
    #[doc = "Bits 0:15 - Y line selection MUX"]
    #[inline(always)]
    pub fn ymux(&self) -> YmuxR {
        YmuxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Y line selection MUX"]
    #[inline(always)]
    #[must_use]
    pub fn ymux(&mut self) -> YmuxW<YselectSpec> {
        YmuxW::new(self, 0)
    }
}
#[doc = "Select Y line\n\nYou can [`read`](crate::Reg::read) this register and get [`yselect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`yselect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YselectSpec;
impl crate::RegisterSpec for YselectSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`yselect::R`](R) reader structure"]
impl crate::Readable for YselectSpec {}
#[doc = "`write(|w| ..)` method takes [`yselect::W`](W) writer structure"]
impl crate::Writable for YselectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets YSELECT to value 0"]
impl crate::Resettable for YselectSpec {
    const RESET_VALUE: u16 = 0;
}
