import { SVGAttributes } from 'react'
import SvgUiElementGenericCommonAbstractProps from './SvgUiElementGenericCommonAbstractProps'

const UpArrow = (props: SvgUiElementGenericCommonAbstractProps & SVGAttributes<SVGSVGElement>) => {
  return (
    <svg
        xmlns="http://www.w3.org/2000/svg" 
        fill={props.fill ?? "none"} 
        viewBox="0 0 24 24" 
        strokeWidth={props.strokeWidth ?? 1} 
        stroke={props.stroke ?? "currentColor"}
        className={props.classname}
        style={props.style}
    >
        <path strokeLinecap="round" strokeLinejoin="round" d="M4.5 15.75l7.5-7.5 7.5 7.5" />
    </svg>

  )
}

export default UpArrow
