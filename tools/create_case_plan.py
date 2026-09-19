"""FreeCAD case study, millimetres. Run: freecad -c tools/create_case_plan.py.
Documented dimensions: Heltec V3.2 datasheet page 14. Other dimensions are estimates.
"""
from pathlib import Path
import math
import FreeCAD as App
import Part

root = Path(__file__).resolve().parents[1]
doc = App.newDocument('UsageDisplayCasePlan')
W, D, H, wall, angle = 76., 50., 38., 5., 25.
lid_drop = wall / math.cos(math.radians(angle))  # True 5 mm panel thickness.
slope = math.tan(math.radians(angle))
rot = App.Rotation(App.Vector(1, 0, 0), angle)
# Board top plane. OLED top is 5 mm above PCB and sits below lid.
# Board stays behind full-thickness wall; cable boot enters the open-top notch.
# OLED glass centre close to x=W/2, y=D/2.
origin = App.Vector(6.5, 15.4, H + 15.4*slope - lid_drop - 6/math.cos(math.radians(angle)))
assembly = App.Placement(origin, rot)
wood = (0.72, .48, .25)
objects = {}


def box(x,y,z,a,b,c):
    return Part.makeBox(a,b,c,App.Vector(x,y,z))


def local(shape):
    shape = shape.copy()
    shape.Placement = assembly.multiply(shape.Placement)
    return shape


def item(name, shape, color, note='Estimated from photo; verify on actual hardware.', group=None):
    obj = doc.addObject('Part::Feature', name)
    obj.Shape = shape
    obj.addProperty('App::PropertyString','Evidence','Planning')
    obj.Evidence = note
    if obj.ViewObject:
        obj.ViewObject.ShapeColor = color
    if group: group.addObject(obj)
    assert shape.isValid(), name
    objects[name] = obj
    return obj


board_group = doc.addObject('App::DocumentObjectGroup','HeltecV32')
case_group = doc.addObject('App::DocumentObjectGroup','WoodenCase')
controls = doc.addObject('App::DocumentObjectGroup','Controls')

# Chamfered PCB silhouette, including the antenna-side tongue.
pts=[(1.3,0),(48,0),(48,5),(50.2,8),(50.2,17.5),(48,20.5),(48,25.5),(1.3,25.5),(0,24),(0,1.5)]
wire=Part.makePolygon([App.Vector(x,y,-1.2) for x,y in pts+[pts[0]]])
pcb=Part.Face(wire).extrude(App.Vector(0,0,1.2))
# 18 contacts per side at official 2.54 mm pitch.
for y in (1.3,24.2):
    for n in range(18):
        pcb=pcb.cut(Part.makeCylinder(.5,2,App.Vector(2.7+n*2.54,y,-1.5)))
item('PCB',local(pcb),(.88,.88,.82),'50.2 x 25.5 mm official overall outline; contour and 1.2 mm thickness estimated.',board_group)
for y,label in [(1.3,'Front'),(24.2,'Rear')]:
    pins=[]; rings=[]
    for n in range(18):
        x=2.7+n*2.54
        pins.append(box(x-.32,y-.32,-9.5,.64,.64,9.7))
        rings.append(Part.makeCylinder(.9,.12,App.Vector(x,y,0)).cut(Part.makeCylinder(.5,.2,App.Vector(x,y,0))))
    item(label+'HeaderPins',local(Part.makeCompound(pins)),(.75,.75,.78),group=board_group)
    item(label+'ContactPads',local(Part.makeCompound(rings)),(.8,.65,.25),group=board_group)
    item(label+'HeaderStrip',local(box(1.5,y-1.2,-3.7,45.8,2.4,2.5)),(.12,.12,.12),group=board_group)

# OLED physical outline, glass and active pixel area are separate.
item('OLEDCarrier',local(box(14.72,3.47,1.2,33.28,18.56,3.2)),(.22,.22,.23),'33.28 x 18.56 mm from official drawing; offset and stack estimate.',board_group)
item('OLEDGlass',local(box(17.72,3.47,4.4,27.28,18.56,.6)),(.04,.07,.08),'27.28 mm glass width from drawing; depth placement estimated.',board_group)
item('OLEDActiveArea',local(box(20.5,6.9,5.01,21.74,10.86,.05)),(.05,.65,.75),'Approximate active region, not the outer glass size.',board_group)
item('OLEDRibbon',local(box(26,1.8,.5,10,5,.3)),(.1,.1,.1),group=board_group)

usb_shell=box(-.8,8.2,.1,8,9.1,3.2).cut(box(-1,8.7,.5,7,8.1,2.4))
item('USB_C_Shell',local(usb_shell),(.75,.76,.78),group=board_group)
item('USB_C_Tongue',local(box(0,9.3,1.4,5,6.8,.5)),(.12,.12,.12),group=board_group)
for y,name in [(21,'PRG'),(4.5,'RST')]:
    item(name+'SwitchBody',local(box(2.2,y-1.7,0,4.5,3.4,1.2)),(.25,.25,.25),group=board_group)
    item(name+'SwitchCap',local(Part.makeCylinder(1,1,App.Vector(4.45,y,1.2))),(.08,.08,.08),group=board_group)
item('BatterySocketRear',local(box(7,9,-4.5,5,6,3.3).cut(box(6.9,9.7,-4,3.5,4.6,2))),(.9,.88,.78),'Rear socket location is unverified; cable route must be checked.',board_group)
item('RFShieldRear',local(box(23,5,-3.5,19,15,2.3)),(.65,.65,.67),group=board_group)
item('AntennaConnector',local(Part.makeCylinder(1.2,1.2,App.Vector(49,12.75,0))),(.8,.67,.25),group=board_group)


def wedge(offset):
    p=[App.Vector(0,0,0),App.Vector(0,D,0),App.Vector(0,D,H+D*slope+offset),App.Vector(0,0,H+offset)]
    return Part.Face(Part.makePolygon(p+[p[0]])).extrude(App.Vector(W,0,0))

outer=wedge(0); lower=wedge(-lid_drop)
body=lower.cut(box(wall,wall,wall,W-2*wall,D-2*wall,100))
lid=outer.cut(lower)
# Real through-opening in lid, transformed in the same coordinates as the OLED.
display_cut=local(box(17.2,2.95,4,28.32,19.6,15))
lid=lid.cut(display_cut)
# PRG bore plus wider captive flange recess underneath. RST remains a service hole.
prg_bore=local(Part.makeCylinder(1.8,20,App.Vector(4.45,21,1)))
prg_recess=local(Part.makeCylinder(3.1,4.2,App.Vector(4.45,21,2.5)))
lid=lid.cut(prg_bore).cut(prg_recess)
lid=lid.cut(local(Part.makeCylinder(1.4,20,App.Vector(4.45,4.5,1))))
# Rectangular open-top notch: two saw cuts and a straight bottom cut.
# 20 mm width provides provisional boot clearance, with no internal pocket.
usb_cut=box(-1,17,32,wall+2,20,70)
body=body.cut(usb_cut)
# Full 5 mm rear wall: no thin snap-in recess. Actual switch retention is unresolved.
body=body.cut(box(34,D-wall-1,20,19,wall+2,6.8))
# Separate short glued blocks, not full-height pillars merged into the walls.
# Magnet axes are normal to the lid. Four magnets, with four steel targets.
normal = rot.multVec(App.Vector(0,0,1))
for index,(x,y) in enumerate([(9,9),(W-9,9),(9,D-9),(W-9,D-9)],1):
    top = App.Vector(x,y,H+y*slope-lid_drop)
    block = box(x-4,y-4,top.z-11,8,8,15).common(lower).cut(wedge(-lid_drop-5/math.cos(math.radians(angle))))
    # Bottom is flat; top follows the lid. Pocket depth includes 0.1 mm glue space.
    pocket = Part.makeCylinder(1.6,1.7,top-normal*1.6,normal)
    block = block.cut(pocket)
    item('MagnetBlock'+str(index),block,wood,'Separate 8 x 8 mm block cut from 5 mm wood, parallel to lid; glue to inside walls. Pocket diameter 3.2 mm, provisional depth 1.6 mm.',case_group)
    item('LidMagnet'+str(index),Part.makeCylinder(1.5,1.5,top-normal*1.5,normal),(.6,.6,.65),'Diameter 3 mm, assumed thickness 1.5 mm. Measure before drilling.',controls)
    target_pocket = Part.makeCylinder(2.1,.6,top,normal)
    lid = lid.cut(target_pocket)
    item('SteelTarget'+str(index),Part.makeCylinder(2,.5,top,normal),(.7,.7,.75),'Additional steel disc, diameter 4 mm x 0.5 mm provisional. Glue flush into lid. Test holding force.',controls)
item('CaseBody',body,wood,'76 x 50 mm; front 38 mm, rear 61.3 mm. Walls/base 5 mm. Open-top USB notch 20 mm wide, bottom at 32 mm. No integral screw pillars.',case_group)
item('SlopedLid',lid,wood,'25 degrees; true 5 mm panel thickness. No screw holes. Four shallow steel-target pockets. OLED/PRG/RST openings retained. Final fit not yet tested.',case_group)
# Existing male pins plug into two 1x18 female headers. No desoldering.
# Closed-bottom sockets insulate pin tips. Buy/measure suitable sockets first.
for y,name in [(1.3,'Front'),(24.2,'Rear')]:
    socket=box(1.43,y-1.27,-10.5,45.72,2.54,6.8)
    for n in range(18):
        socket=socket.cut(box(2.7+n*2.54-.45,y-.45,-9.8,.9,.9,6.4))
    item(name+'MountSocket',local(socket),(.14,.14,.14),'Proposed 1x18 female header, 2.54 mm pitch, 7 mm body. Model omits contacts/tails. Insulate all tails; check insertion depth and actual header spacing before buying.',controls)
    # Five mm wooden carrier extends to both inner side walls.
    rail=local(box(-1.5,y-2.5,-15.5,66,5,5)).common(box(wall,wall,wall,W-2*wall,D-2*wall,100))
    item(name+'SocketCarrier',rail,wood,'Separate 5 mm wooden carrier; glue ends to side walls. Fix insulated sockets to carrier with epoxy. Insert board only after cure; keep glue out of contacts. Dry-fit before gluing.',case_group)

battery=item('LiPo102050',box(12,11,6,50,20,10),(.72,.2,.2),'Purchased candidate: 50 x 20 x 10 mm, 1000 mAh. Leave clearance; never compress pouch.')
item('BatteryTray',box(10,9,5,54,24,2).cut(box(12,11,6,50,20,2)),(.2,.2,.2),'Insulating tray concept. Add gentle removable retention, not screws through pouch.',case_group)
item('BatteryProtectionEnd',box(12,11,16,5,20,1),(.85,.7,.2),'Allowance for protection PCB/tape; not an exact battery model.')
# Power switch shown installed through rear wall with a rocker and terminals.
item('PowerSwitchBody',box(34,D-12,20,19,12,6.8),(.12,.12,.12),'19 x 6.8 mm nominal opening. Body depth estimated. Snap-in compatibility with 5 mm wood unverified; do not thin wall to fit. Retention or alternative switch still required.',controls)
item('PowerSwitchBezel',box(33,D,19,21,1.5,8.8),(.1,.1,.1),group=controls)
item('PowerRocker',box(35,D+1.5,20,17,2,6.8),(.22,.22,.22),group=controls)
item('PowerTerminals',Part.makeCompound([box(x,D-16,22,2,4,.5) for x in (37,47)]),(.75,.75,.75),group=controls)
# Flange catches on lid underside; small gap to board button avoids permanent pressure.
plunger=Part.makeCylinder(1.55,9.5,App.Vector(4.45,21,2.4)).fuse(Part.makeCylinder(2.8,1,App.Vector(4.45,21,5.5)))
item('CaptivePRGPlunger',local(plunger),(.8,.7,.5),'Wood/plastic, 0.2 mm initial gap. Flange retains knob; travel and return must be tested.',controls)

notes=doc.addObject('App::FeaturePython','DesignNotes')
for key,value in {
 'Sources':'Heltec V3.2 datasheet p14 + supplied front/side photos. Official STEP V3.7 checked: 51.69 mm PCB, no OLED; not used as exact V3.2 geometry.',
 'Confirmed':'Overall 50.2 x 25.5 x 10.2 mm; pin pitch 2.54 mm; OLED drawing 33.28 x 18.56 mm, glass width 27.28 mm.',
 'Estimated':'PCB contour, component positions/heights, header length, rear connector, switch body, supports and all machining tolerances.',
 'Power':'Switch in battery positive adapter wire. USB powers board with switch off; battery cannot charge while disconnected.',
 'Status':'Planning model, NOT a cutting template. Separate magnet blocks and steel targets replace screw pillars. Existing pins retained; two insulated 1x18 sockets on 5 mm carriers hold board. Socket dimensions and strength unverified. Rocker retention in 5 mm wood unresolved.',
}.items():
 notes.addProperty('App::PropertyString',key,'Planning'); setattr(notes,key,value)
# Check major occupied volumes, excluding intentional mating contacts.
for a,b in [('LiPo102050','PCB'),('LiPo102050','FrontHeaderPins'),('LiPo102050','RearHeaderPins'),('LiPo102050','CaseBody'),('OLEDGlass','SlopedLid'),('PowerSwitchBody','CaseBody'),('CaptivePRGPlunger','SlopedLid'),('PCB','CaseBody'),('FrontHeaderPins','CaseBody'),('RearHeaderPins','CaseBody'),('PowerSwitchBody','PCB'),('PowerTerminals','RearHeaderPins'),('PowerTerminals','LiPo102050')]:
 v=objects[a].Shape.common(objects[b].Shape).Volume
 assert v<.001,(a,b,v)
# Check every solid pair; only pins embedded in their original plastic strips overlap.
from itertools import combinations
allowed = {frozenset((name+'HeaderPins', name+'HeaderStrip')) for name in ('Front','Rear')}
for a,b in combinations(objects.values(),2):
 if frozenset((a.Name,b.Name)) not in allowed:
  assert a.Shape.common(b.Shape).Volume < .001, (a.Name,b.Name,'overlap')
# Provisional cable boot plus straight insertion path through the left wall.
cable_path = local(box(-18,6,-1,17.2,13.5,7))
for shell in (body, lid):
 assert cable_path.common(shell).Volume < .001, 'USB insertion path blocked'
doc.recompute()
if App.GuiUp:
 import FreeCADGui as Gui
 Gui.activeDocument().activeView().viewAxonometric(); Gui.activeDocument().activeView().fitAll()
doc.saveAs(str(root/'case-plan.FCStd'))
print('PASS: valid shapes; selected component collision checks passed.')
App.closeDocument(doc.Name)
check=App.openDocument(str(root/'case-plan.FCStd'))
assert all(o.Shape.isValid() for o in check.Objects if hasattr(o,'Shape'))
print('PASS: saved file reopened with',len(check.Objects),'objects.')
App.closeDocument(check.Name)
