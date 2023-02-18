import SvgUiElementGenericCommonAbstractProps from './SvgUiElementGenericCommonAbstractProps'

const RightButton = (props: SvgUiElementGenericCommonAbstractProps) => {
  return (
    <svg 
        xmlns="http://www.w3.org/2000/svg" 
        fill={props.fill ?? "none"} 
        viewBox="0 0 24 24" 
        strokeWidth={props.strokeWidth ?? 1} 
        stroke={props.stroke ?? "currentColor"}
        className={props.classname}
    >
        <path strokeLinecap="round" strokeLinejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
    </svg>

  )
}

export default RightButton