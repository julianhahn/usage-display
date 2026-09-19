"""Create an A4 woodworking draft: three 1:1 drawing pages and a 3D reference page.
Run: python3 tools/create_case_drawing.py. All dimensions are millimetres.
This is a proposed panel construction, not a verified manufacturing export.
"""
from pathlib import Path
from math import tan, cos, radians
from reportlab.pdfgen import canvas
from reportlab.lib.units import mm
from reportlab.lib.colors import HexColor

root = Path(__file__).resolve().parents[1]
out = root / 'case-plan-2d-draft.pdf'
c = canvas.Canvas(str(out), pagesize=(297*mm,210*mm))
c.setTitle('Usage Display - woodworking draft - 5 mm wood')
a = radians(25)
slope = tan(a)
low = 38-5/cos(a)
high = low+50*slope


def text(x,y,s,size=9):
    c.setFillColor(HexColor('#202830')); c.setFont('Helvetica',size)
    c.drawString(x*mm,y*mm,s)


def line(x,y,u,v):
    c.line(x*mm,y*mm,u*mm,v*mm)


def polygon(x,y,points):
    c.setStrokeColor(HexColor('#202830')); c.setLineWidth(.7)
    p=c.beginPath(); p.moveTo((x+points[0][0])*mm,(y+points[0][1])*mm)
    for u,v in points[1:]: p.lineTo((x+u)*mm,(y+v)*mm)
    p.close(); c.drawPath(p)


def rect(x,y,w,h): polygon(x,y,[(0,0),(w,0),(w,h),(0,h)])


def hd(x,y,w,label):
    c.setLineWidth(.4); line(x,y,x+w,y)
    for u in (x,x+w): line(u,y-1.4,u,y+1.4)
    text(x+w/2-len(label)*.8,y+2,label,8)


def vd(x,y,h,label):
    line(x,y,x,y+h)
    for v in (y,y+h): line(x-1.4,v,x+1.4,v)
    text(x+2,y+h/2,label,8)


def header(page,title):
    text(15,195,title,16)
    text(15,186,'DRAFT - NOT RELEASED FOR CUTTING. Millimetres. Print at 100%, never Fit to page.',9)
    text(15,8,f'Usage Display | Page {page}/4 | Based on 76 x 50 mm case, 25-degree lid, 5 mm stock.',8)
    hd(222,12,50,'50 mm print check')


text(15,195,'3D assembly reference - lid removed',16)
text(15,186,'Visual guide only - not to scale. Use the dimensions and draft warnings on pages 2-4.',9)
c.drawImage(str(root / 'docs/case/assembly-reference.png'), 57*mm, 31*mm,
            width=183*mm, height=150*mm, preserveAspectRatio=True, anchor='c')
text(15,23,'Four magnet blocks hold the lid. Two socket carriers support the board; its pins stay fitted.',9)
text(15,16,'User-supplied view. Component fit and mounting details still need checks on the real hardware.',9)
text(15,8,'Usage Display | Page 1/4 | Assembly reference - not a cutting template.',8)
c.showPage()
header(2,'Wooden case: side panels and assembly')
text(18,171,'A - SIDE PANELS: 2 mirrored pieces, 5 mm thick',11)
x,y=30,93
polygon(x,y,[(0,0),(50,0),(50,high),(0,low)])
hd(x,y-8,50,'50.00')
vd(x-7,y,low,f'{low:.2f}')
vd(x+56,y,high,f'{high:.2f}')
text(x+13,y+low+8,'25 degrees',9)
text(x-3,y-17,'FRONT'); text(x+39,y-17,'REAR')
text(112,166,'Outer size: 76 wide x 50 deep.',10)
text(112,159,'Height with lid: 38 front / 61.32 rear.',10)
text(112,150,'Sides run the full depth.',10)
text(112,143,'Front and rear fit BETWEEN the sides.',10)
text(112,136,'Bottom fits inside all four walls.',10)
text(112,127,'Glue joints are proposed butt joints.',10)
text(112,120,'Use square cuts unless a bevel is labelled.',10)
text(112,111,'Top outline is a 25-degree slope, not an edge bevel.',9)
text(112,102,'Round outside edges only after dry fitting.',9)
text(18,62,'USB notch - LEFT SIDE ONLY',11)
text(18,54,'Draft position: starts 17 mm from front, 20 mm wide; bottom 32 mm above base.',9)
text(18,47,'Open from the sloped top edge. Two downward saw cuts plus a straight bottom cut.',9)
text(18,40,'DO NOT CUT YET: seat the real board, plug in your cable, then mark the clearance.',9)
# Dashed USB notch is a reference overlay, not an approved cut line.
c.setDash(3,2); polygon(x,y,[(17,32),(37,32),(37,low+37*slope),(17,low+17*slope)])
c.setDash()
text(18,28,'Measure stock first. This drawing assumes exactly 5.00 mm finished thickness.',9)
c.showPage()

header(3,'Wooden case: front, rear and bottom')
text(18,172,'B - FRONT: 1 piece',11)
rect(22,110,66,low+5*slope)
hd(22,102,66,'66.00')
vd(92,110,low+5*slope,f'{low+5*slope:.2f} blank')
text(18,91,f'Plane top to 25 degrees: outside {low:.2f}, inside {low+5*slope:.2f} high.',8)
text(160,172,'C - REAR: 1 piece',11)
rect(160,110,66,high)
hd(160,102,66,'66.00')
text(160,91,f'Blank height {high:.2f}. Outside high edge.',9)
text(160,84,f'Inside top height {high-5*slope:.2f}. Top bevel 25 degrees.',8)
text(18,75,'D - BOTTOM: 1 piece, 5 mm thick',11)
rect(22,26,66,40)
hd(22,21,66,'66.00'); vd(93,26,40,'40.00')
text(133,68,'Rear rocker opening: nominal 19 x 6.8 mm.',10)
text(133,60,'Mark ONLY after checking the real switch.',9)
text(133,52,'Its clips may not grip 5 mm wood.',9)
text(133,44,'Do not thin the wall to make it fit.',9)
text(133,32,'Leave a small fitting allowance on blanks.',9)
text(133,25,'Plane to the finished dimensions after dry fitting.',9)
c.showPage()

header(4,'Removable lid, magnets and board mounting')
lid_length=50/cos(a)
text(18,173,'E - LID: 1 piece, true thickness 5 mm',11)
rect(25,104,76,lid_length)
hd(25,97,76,'76.00'); vd(107,104,lid_length,f'{lid_length:.2f}')
text(25,88,'FRONT EDGE',8)
text(132,166,'Diagram is the OUTSIDE lid face.',10)
text(132,158,'Front/rear edges: 25-degree bevel from square.',9)
text(132,150,'Offset across 5 mm thickness: 2.33 mm.',9)
text(132,142,'Measure blank length on ONE face, not tip to tip.',9)
text(132,132,'Simpler option: leave lid edges square first.',9)
text(132,124,'This changes the edge overhang, not the slope.',9)
# Draft OLED opening projected onto outer lid plane, matching script placement.
origin_y=15.4
origin_z=38+origin_y*slope-5/cos(a)-6/cos(a)
# Coordinates along slope measured from outside front edge at (y=0,z=38).
base_s=origin_y*cos(a)+(origin_z-38)*__import__('math').sin(a)
ox=6.5+17.2; oy=base_s+2.95
c.setDash(3,2); rect(25+ox,104+oy,28.32,19.6); c.setDash()
text(132,111,'Dashed OLED opening: 28.32 x 19.60, provisional.',9)
text(132,103,'Mark from mounted display before drilling/sawing.',9)
text(18,75,'MAGNETS - no screw holes',11)
text(18,67,'4 separate 8 x 8 mm blocks from 5 mm wood. Glue inside corners, tops parallel to lid.',9)
text(18,60,'4 magnets: diameter 3 mm; assumed thickness 1.5 mm. Measure first.',9)
text(18,53,'Draft pockets: diameter 3.2 mm, depth 1.6 mm. Test on scrap; use a drill depth stop.',9)
text(18,46,'4 steel targets under lid: proposed diameter 4 mm x 0.5 mm. Set flush; test holding force.',9)
text(18,35,'BOARD MOUNTING - fit before cutting carriers',11)
text(18,27,'Keep male pins. Fit two insulated 1x18 female sockets, 2.54 mm pitch, on 5 mm wooden carriers.',9)
text(18,20,'Carrier lengths, glue positions, PRG plunger and lid locating stops must be set from the real assembly.',8)
c.save()
print(out)
